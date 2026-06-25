use std::io;
use std::net::Ipv4Addr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers, MouseButton, MouseEventKind};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, Gauge, List, ListItem, Paragraph, Tabs, Wrap},
    Frame,
};

use crate::libs::calc_ip::{calculate_subnet, generate_subnets};
use crate::libs::scanner::{self, ScanResult};

const MAX_HISTORY: usize = 10;

#[derive(Clone, Copy, PartialEq)]
enum ActiveTab {
    Calculator,
    Subnets,
    Scanner,
}

enum AppEvent {
    ScanDone(Result<ScanResult, String>),
}

pub struct App {
    should_quit: bool,
    active_tab: ActiveTab,
    focus_idx: usize,
    show_confirm: bool,
    confirm_msg: String,
    pending_scan_cidr: Option<String>,

    // history
    calc_history: Vec<String>,
    calc_history_idx: Option<usize>,
    calc_history_saved: String,
    scan_history: Vec<String>,
    scan_history_idx: Option<usize>,
    scan_history_saved: String,
    sub_history: Vec<String>,
    sub_history_idx: Option<usize>,
    sub_history_saved: String,

    // calculator
    calc_input: String,
    calc_result: String,
    calc_scroll: usize,

    // subnets
    sub_cidr: String,
    sub_prefix: String,
    sub_filter: String,
    sub_result: String,
    sub_scroll: usize,
    sub_total: u32,

    // scanner
    scan_input: String,
    is_scanning: bool,
    scan_completed: Arc<AtomicUsize>,
    scan_results: Vec<Ipv4Addr>,
    scan_total: usize,
    scan_error: String,
    scan_scroll: usize,
    event_rx: Option<mpsc::Receiver<AppEvent>>,

    term_width: u16,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            active_tab: ActiveTab::Calculator,
            focus_idx: 0,
            show_confirm: false,
            confirm_msg: String::new(),
            pending_scan_cidr: None,

            calc_history: Vec::new(),
            calc_history_idx: None,
            calc_history_saved: String::new(),
            scan_history: Vec::new(),
            scan_history_idx: None,
            scan_history_saved: String::new(),
            sub_history: Vec::new(),
            sub_history_idx: None,
            sub_history_saved: String::new(),

            calc_input: String::new(),
            calc_result: String::new(),
            calc_scroll: 0,

            sub_cidr: String::new(),
            sub_prefix: String::new(),
            sub_filter: String::new(),
            sub_result: String::new(),
            sub_scroll: 0,
            sub_total: 0,

            scan_input: String::new(),
            is_scanning: false,
            scan_completed: Arc::new(AtomicUsize::new(0)),
            scan_results: Vec::new(),
            scan_total: 0,
            scan_error: String::new(),
            scan_scroll: 0,
            event_rx: None,
            term_width: 0,
        }
    }

    pub fn run(&mut self, terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<io::Stdout>>) -> io::Result<()> {
        crossterm::execute!(io::stdout(), crossterm::event::EnableMouseCapture)?;
        while !self.should_quit {
            terminal.draw(|f| self.render(f))?;
            self.handle_events()?;
        }
        crossterm::execute!(io::stdout(), crossterm::event::DisableMouseCapture)?;
        Ok(())
    }

    // ── rendering ──────────────────────────────────────────────

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        self.term_width = area.width;
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(2),
            ])
            .split(area);

        self.render_title(frame, chunks[0]);
        self.render_tabs(frame, chunks[1]);
        self.render_content(frame, chunks[2]);
        self.render_help(frame, chunks[3]);

        if self.show_confirm {
            self.render_confirm(frame, area);
        }
    }

    fn render_title(&self, frame: &mut Frame, area: Rect) {
        let title = Paragraph::new(Line::from(vec![
            Span::styled(" IP Calculator", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]));
        frame.render_widget(title, area);
    }

    fn render_tabs(&self, frame: &mut Frame, area: Rect) {
        let titles = vec![" Calculator ", " Subnets ", " Scanner "];
        let current = match self.active_tab {
            ActiveTab::Calculator => 0,
            ActiveTab::Subnets => 1,
            ActiveTab::Scanner => 2,
        };
        let tabs = Tabs::new(titles)
            .select(current)
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Plain))
            .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(tabs, area);
    }

    fn render_content(&self, frame: &mut Frame, area: Rect) {
        match self.active_tab {
            ActiveTab::Calculator => self.render_calculator(frame, area),
            ActiveTab::Subnets => self.render_subnets(frame, area),
            ActiveTab::Scanner => self.render_scanner(frame, area),
        }
    }

    fn render_input_block<'a>(&self, label: &str, value: &'a str, focused: bool, placeholder: &'a str) -> Paragraph<'a> {
        let border_style = if focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        let inner = if value.is_empty() {
            Text::from(Line::from(vec![
                Span::styled(
                    format!("{}: ", label),
                    Style::default().fg(if focused { Color::Cyan } else { Color::DarkGray }),
                ),
                Span::styled(placeholder, Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC)),
            ]))
        } else {
            Text::from(Line::from(vec![
                Span::styled(
                    format!("{}: ", label),
                    Style::default().fg(if focused { Color::Cyan } else { Color::DarkGray }),
                ),
                Span::raw(value),
            ]))
        };
        Paragraph::new(inner).block(Block::default().borders(Borders::ALL).border_style(border_style))
    }

    fn render_button<'a>(&self, label: &str, focused: bool) -> Paragraph<'a> {
        let style = if focused {
            Style::default().fg(Color::White).bg(Color::Blue).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        Paragraph::new(Line::from(Span::styled(format!(" {} ", label), style)))
            .block(Block::default().borders(Borders::ALL).border_style(if focused {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default().fg(Color::DarkGray)
            }))
    }

    fn render_calculator(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Min(5)])
            .margin(1)
            .split(area);

        let focused_input = self.active_tab == ActiveTab::Calculator && self.focus_idx == 0;
        frame.render_widget(
            self.render_input_block("CIDR", &self.calc_input, focused_input, "e.g. 192.168.1.0/24"),
            chunks[0],
        );

        let focused_btn = self.active_tab == ActiveTab::Calculator && self.focus_idx == 1;
        frame.render_widget(self.render_button("Display", focused_btn), chunks[1]);

        if !self.calc_result.is_empty() {
            let result = Paragraph::new(self.calc_result.as_str())
                .block(Block::default().borders(Borders::ALL).title(" Result ").border_type(BorderType::Rounded))
                .scroll((self.calc_scroll as u16, 0))
                .wrap(Wrap { trim: false });
            frame.render_widget(result, chunks[2]);
        }
    }

    fn render_subnets(&self, frame: &mut Frame, area: Rect) {
        let outer = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(5)])
            .margin(1)
            .split(area);
        let input_area = outer[0];
        let result_area = outer[1];

        let fields = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Percentage(25),
                Constraint::Percentage(20),
                Constraint::Percentage(15),
            ])
            .split(input_area);

        let f_cidr = self.active_tab == ActiveTab::Subnets && self.focus_idx == 0;
        let f_prefix = self.active_tab == ActiveTab::Subnets && self.focus_idx == 1;
        let f_filter = self.active_tab == ActiveTab::Subnets && self.focus_idx == 2;
        let f_btn = self.active_tab == ActiveTab::Subnets && self.focus_idx == 3;

        frame.render_widget(self.render_input_block("CIDR", &self.sub_cidr, f_cidr, "192.168.1.0/24"), fields[0]);
        frame.render_widget(self.render_input_block("Prefix", &self.sub_prefix, f_prefix, "e.g. 26"), fields[1]);
        frame.render_widget(self.render_input_block("Filter", &self.sub_filter, f_filter, "max subnets"), fields[2]);
        frame.render_widget(self.render_button("Generate", f_btn), fields[3]);

        if !self.sub_result.is_empty() {
            let summary = Line::from(vec![
                Span::styled(format!("Total subnets: {}", self.sub_total), Style::default().fg(Color::Green)),
            ]);
            let content = format!("{}\n\n{}", summary, self.sub_result);
            let result = Paragraph::new(content)
                .block(Block::default().borders(Borders::ALL).title(" Subnets ").border_type(BorderType::Rounded))
                .scroll((self.sub_scroll as u16, 0))
                .wrap(Wrap { trim: false });
            frame.render_widget(result, result_area);
        }
    }

    fn render_scanner(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Min(5)])
            .margin(1)
            .split(area);

        let focused_input = self.active_tab == ActiveTab::Scanner && self.focus_idx == 0;
        frame.render_widget(
            self.render_input_block("CIDR", &self.scan_input, focused_input, "e.g. 192.168.1.0/24"),
            chunks[0],
        );

        if self.is_scanning {
            let completed = self.scan_completed.load(Ordering::Relaxed);
            let ratio = if self.scan_total > 0 {
                completed as f64 / self.scan_total as f64
            } else {
                0.0
            };
            let gauge = Gauge::default()
                .block(Block::default().borders(Borders::ALL).title(" Scanning "))
                .gauge_style(Style::default().fg(Color::Cyan))
                .ratio(ratio)
                .label(format!("{}/{}", completed, self.scan_total));
            frame.render_widget(gauge, chunks[1]);
        } else if !self.scan_error.is_empty() {
            let error = Paragraph::new(self.scan_error.as_str()).style(Style::default().fg(Color::Red));
            frame.render_widget(error, chunks[1]);
        } else if !self.scan_results.is_empty() {
            let summary = Paragraph::new(Line::from(vec![
                Span::styled(
                    format!("{} responsive hosts", self.scan_results.len()),
                    Style::default().fg(Color::Green),
                ),
            ]));
            frame.render_widget(summary, chunks[1]);
        } else {
            let focused_btn = self.active_tab == ActiveTab::Scanner && self.focus_idx == 1;
            frame.render_widget(self.render_button("Scan", focused_btn), chunks[1]);
        }

        if !self.scan_error.is_empty() && self.scan_results.is_empty() {
            let error = Paragraph::new(self.scan_error.as_str()).style(Style::default().fg(Color::Red));
            frame.render_widget(error, chunks[2]);
        } else if !self.scan_results.is_empty() {
            let items: Vec<ListItem> = self
                .scan_results
                .iter()
                .map(|ip| {
                    ListItem::new(Line::from(vec![
                        Span::styled("  ", Style::default().fg(Color::DarkGray)),
                        Span::styled("\u{25cf}", Style::default().fg(Color::Green)),
                        Span::raw(format!(" {}", ip)),
                    ]))
                })
                .collect();
            let list = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!(" Responsive hosts ({}) ", self.scan_results.len()))
                        .border_type(BorderType::Rounded),
                )
                .highlight_style(Style::default().fg(Color::Cyan));
            frame.render_widget(list, chunks[2]);
        }
    }

    fn render_confirm(&self, frame: &mut Frame, area: Rect) {
        let popup = Rect {
            x: area.width / 4,
            y: area.height / 3,
            width: area.width / 2,
            height: 7,
        };
        frame.render_widget(Clear, popup);
        let lines = vec![
            Line::from(vec![Span::styled(" Warning ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))]),
            Line::from(""),
            Line::from(self.confirm_msg.as_str()),
            Line::from(""),
            Line::from(vec![
                Span::styled(" [Enter] ", Style::default().fg(Color::Green)),
                Span::styled("Confirm  ", Style::default().fg(Color::DarkGray)),
                Span::styled(" [Esc] ", Style::default().fg(Color::Yellow)),
                Span::styled("Cancel", Style::default().fg(Color::DarkGray)),
            ]),
        ];
        let para = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Yellow)))
            .alignment(Alignment::Center);
        frame.render_widget(para, popup);
    }

    fn render_help(&self, frame: &mut Frame, area: Rect) {
        let text = Line::from(vec![
            Span::styled(" Tab ", Style::default().fg(Color::Yellow)),
            Span::styled("Tab  ", Style::default().fg(Color::DarkGray)),
            Span::styled(" Enter ", Style::default().fg(Color::Yellow)),
            Span::styled("Go  ", Style::default().fg(Color::DarkGray)),
            Span::styled(" j/k ", Style::default().fg(Color::Yellow)),
            Span::styled("Scroll  ", Style::default().fg(Color::DarkGray)),
            Span::styled(" \u{2191}/\u{2193} ", Style::default().fg(Color::Yellow)),
            Span::styled("History  ", Style::default().fg(Color::DarkGray)),
            Span::styled(" Esc/q ", Style::default().fg(Color::Yellow)),
            Span::styled("Quit", Style::default().fg(Color::DarkGray)),
        ]);
        let help = Paragraph::new(text).block(Block::default().borders(Borders::TOP));
        frame.render_widget(help, area);
    }

    // ── events ─────────────────────────────────────────────────

    fn handle_events(&mut self) -> io::Result<()> {
        self.check_scan_done();

        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    self.handle_key(key.code, key.modifiers);
                }
                Event::Mouse(m) if !self.show_confirm => {
                    if matches!(m.kind, MouseEventKind::Down(MouseButton::Left)) {
                        self.handle_mouse(m.column, m.row);
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, code: KeyCode, _mods: KeyModifiers) {
        if self.show_confirm {
            match code {
                KeyCode::Enter => {
                    if let Some(cidr) = self.pending_scan_cidr.take() {
                        self.show_confirm = false;
                        self.scan_input = cidr;
                        self.execute_scan();
                    }
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.show_confirm = false;
                    self.pending_scan_cidr = None;
                }
                _ => {}
            }
            return;
        }

        match code {
            KeyCode::Esc | KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Tab => self.focus_next(),
            KeyCode::BackTab => self.focus_prev(),
            KeyCode::Enter => self.activate_focused(),
            KeyCode::Backspace => self.input_pop(),
            KeyCode::Char(c) if !c.is_control() => self.input_push(c),
            KeyCode::Up => self.nav_up(),
            KeyCode::Down => self.nav_down(),
            _ => {}
        }
    }

    fn handle_mouse(&mut self, col: u16, row: u16) {
        if row == 1 {
            let inner = self.term_width.saturating_sub(2);
            let third = inner / 3;
            if col < 1 + third {
                self.active_tab = ActiveTab::Calculator;
            } else if col < 1 + 2 * third {
                self.active_tab = ActiveTab::Subnets;
            } else {
                self.active_tab = ActiveTab::Scanner;
            }
            self.focus_idx = 0;
        }
    }

    // ── focus helpers ──────────────────────────────────────────

    fn focus_count(&self) -> usize {
        match self.active_tab {
            ActiveTab::Calculator => 2,
            ActiveTab::Subnets => 4,
            ActiveTab::Scanner => 2,
        }
    }

    fn focus_next(&mut self) {
        let n = self.focus_count();
        if self.focus_idx + 1 < n {
            self.focus_idx += 1;
        } else {
            self.focus_idx = 0;
            self.active_tab = match self.active_tab {
                ActiveTab::Calculator => ActiveTab::Subnets,
                ActiveTab::Subnets => ActiveTab::Scanner,
                ActiveTab::Scanner => ActiveTab::Calculator,
            };
        }
    }

    fn focus_prev(&mut self) {
        if self.focus_idx > 0 {
            self.focus_idx -= 1;
        } else {
            self.active_tab = match self.active_tab {
                ActiveTab::Calculator => ActiveTab::Scanner,
                ActiveTab::Subnets => ActiveTab::Calculator,
                ActiveTab::Scanner => ActiveTab::Subnets,
            };
            self.focus_idx = self.focus_count() - 1;
        }
    }

    fn input_field(&mut self) -> Option<&mut String> {
        match (self.active_tab, self.focus_idx) {
            (ActiveTab::Calculator, 0) => Some(&mut self.calc_input),
            (ActiveTab::Subnets, 0) => Some(&mut self.sub_cidr),
            (ActiveTab::Subnets, 1) => Some(&mut self.sub_prefix),
            (ActiveTab::Subnets, 2) => Some(&mut self.sub_filter),
            (ActiveTab::Scanner, 0) => Some(&mut self.scan_input),
            _ => None,
        }
    }

    fn history_for_input(&mut self) -> Option<HistoryHelper<'_>> {
        match (self.active_tab, self.focus_idx) {
            (ActiveTab::Calculator, 0) => Some(HistoryHelper {
                history: &mut self.calc_history,
                idx: &mut self.calc_history_idx,
                saved: &mut self.calc_history_saved,
                input: &mut self.calc_input,
            }),
            (ActiveTab::Subnets, 0) => Some(HistoryHelper {
                history: &mut self.sub_history,
                idx: &mut self.sub_history_idx,
                saved: &mut self.sub_history_saved,
                input: &mut self.sub_cidr,
            }),
            (ActiveTab::Scanner, 0) => Some(HistoryHelper {
                history: &mut self.scan_history,
                idx: &mut self.scan_history_idx,
                saved: &mut self.scan_history_saved,
                input: &mut self.scan_input,
            }),
            _ => None,
        }
    }

    fn save_history(input: &str, history: &mut Vec<String>, idx: &mut Option<usize>) {
        if input.is_empty() {
            return;
        }
        if history.first().map_or(false, |h| h == input) {
            return;
        }
        history.insert(0, input.to_string());
        if history.len() > MAX_HISTORY {
            history.pop();
        }
        *idx = None;
    }

    // ── input ──────────────────────────────────────────────────

    fn input_push(&mut self, c: char) {
        if let Some(field) = self.input_field() {
            field.push(c);
        }
    }

    fn input_pop(&mut self) {
        if let Some(field) = self.input_field() {
            field.pop();
        }
    }

    // ── navigation ─────────────────────────────────────────────

    fn nav_up(&mut self) {
        if self.history_for_input().is_some() {
            if let Some(mut h) = self.history_for_input() {
                h.go_back();
            }
            return;
        }
        match self.active_tab {
            ActiveTab::Calculator => self.calc_scroll = self.calc_scroll.saturating_sub(1),
            ActiveTab::Subnets => self.sub_scroll = self.sub_scroll.saturating_sub(1),
            ActiveTab::Scanner => self.scan_scroll = self.scan_scroll.saturating_sub(1),
        }
    }

    fn nav_down(&mut self) {
        if self.history_for_input().is_some() {
            if let Some(mut h) = self.history_for_input() {
                h.go_forward();
            }
            return;
        }
        match self.active_tab {
            ActiveTab::Calculator => self.calc_scroll = self.calc_scroll.saturating_add(1),
            ActiveTab::Subnets => self.sub_scroll = self.sub_scroll.saturating_add(1),
            ActiveTab::Scanner => self.scan_scroll = self.scan_scroll.saturating_add(1),
        }
    }

    // ── actions ────────────────────────────────────────────────

    fn activate_focused(&mut self) {
        match (self.active_tab, self.focus_idx) {
            (ActiveTab::Calculator, _) => {
                Self::save_history(&self.calc_input, &mut self.calc_history, &mut self.calc_history_idx);
                self.calc_execute();
            }
            (ActiveTab::Subnets, _) => {
                Self::save_history(&self.sub_cidr, &mut self.sub_history, &mut self.sub_history_idx);
                self.subnets_generate();
            }
            (ActiveTab::Scanner, _) => {
                Self::save_history(&self.scan_input, &mut self.scan_history, &mut self.scan_history_idx);
                self.scan_start();
            }
        }
    }

    fn calc_execute(&mut self) {
        self.calc_result.clear();
        self.calc_scroll = 0;
        match calculate_subnet(&self.calc_input) {
            Ok(subnet) => {
                let first = subnet.first_usable.map_or("N/A".to_string(), |ip| ip.to_string());
                let last = subnet.last_usable.map_or("N/A".to_string(), |ip| ip.to_string());
                self.calc_result = format!(
                    "Network:    {}\n\
                     Mask:       {}\n\
                     CIDR:       /{}\n\
                     Broadcast:  {}\n\
                     First:      {}\n\
                     Last:       {}\n\
                     Hosts:      {}",
                    subnet.network, subnet.mask, subnet.prefix, subnet.broadcast, first, last, subnet.num_hosts
                );
            }
            Err(e) => {
                self.calc_result = format!("Error: {}", e);
            }
        }
    }

    fn subnets_generate(&mut self) {
        self.sub_result.clear();
        self.sub_scroll = 0;
        let prefix: u8 = match self.sub_prefix.parse() {
            Ok(p) => p,
            Err(_) => {
                self.sub_result = "Error: Invalid prefix".to_string();
                return;
            }
        };
        let filter: Option<usize> = if self.sub_filter.is_empty() {
            None
        } else {
            match self.sub_filter.parse() {
                Ok(f) => Some(f),
                Err(_) => {
                    self.sub_result = "Error: Invalid filter".to_string();
                    return;
                }
            }
        };
        match generate_subnets(&self.sub_cidr, prefix, filter, Some(1)) {
            Ok((subnets, total, page, pages)) => {
                self.sub_total = total;
                let mut out = String::new();
                out.push_str(&format!("Page {}/{}:\n\n", page, pages));
                for sn in &subnets {
                    out.push_str(&format!(
                        "Network: {} /{}\n  Mask: {}  Broadcast: {}\n  Hosts: {}  Range: {} - {}\n\n",
                        sn.network,
                        sn.prefix,
                        sn.mask,
                        sn.broadcast,
                        sn.num_hosts,
                        sn.first_usable.map_or("N/A".to_string(), |ip| ip.to_string()),
                        sn.last_usable.map_or("N/A".to_string(), |ip| ip.to_string()),
                    ));
                }
                self.sub_result = out;
            }
            Err(e) => {
                self.sub_result = format!("Error: {}", e);
            }
        }
    }

    fn scan_start(&mut self) {
        let cidr = self.scan_input.clone();
        if cidr.is_empty() {
            return;
        }

        if let Some(total) = estimate_hosts(&cidr) {
            if total > 1000 {
                self.show_confirm = true;
                self.confirm_msg = format!(
                    "Scanning {} will check ~{} hosts.\nThis may take a while. Continue?",
                    cidr, total
                );
                self.pending_scan_cidr = Some(cidr);
                return;
            }
        }
        self.execute_scan();
    }

    fn execute_scan(&mut self) {
        let cidr = self.scan_input.clone();
        if cidr.is_empty() {
            return;
        }

        let completed = Arc::new(AtomicUsize::new(0));
        self.scan_completed = Arc::clone(&completed);
        self.is_scanning = true;
        self.scan_results.clear();
        self.scan_total = 0;
        self.scan_error.clear();
        self.scan_scroll = 0;

        let (tx, rx) = mpsc::channel();
        self.event_rx = Some(rx);

        thread::spawn(move || {
            let result = scanner::scan_subnet_with_progress(&cidr, completed);
            let _ = tx.send(AppEvent::ScanDone(result.map_err(|e| e.to_string())));
        });
    }

    fn check_scan_done(&mut self) {
        if let Some(ref rx) = self.event_rx {
            if let Ok(event) = rx.try_recv() {
                match event {
                    AppEvent::ScanDone(result) => {
                        self.is_scanning = false;
                        match result {
                            Ok(sr) => {
                                self.scan_results = sr.responsive_hosts;
                                self.scan_total = sr.total_scanned;
                            }
                            Err(e) => self.scan_error = e,
                        }
                    }
                }
            }
        }
    }
}

fn estimate_hosts(cidr: &str) -> Option<usize> {
    let parts: Vec<&str> = cidr.split('/').collect();
    if parts.len() != 2 {
        return None;
    }
    let prefix: u8 = parts[1].parse().ok()?;
    let hosts = match prefix {
        32 => 1,
        31 => 2,
        0 => u32::MAX - 1,
        _ => (1u32 << (32 - prefix)) - 2,
    };
    Some(hosts as usize)
}

struct HistoryHelper<'a> {
    history: &'a mut Vec<String>,
    idx: &'a mut Option<usize>,
    saved: &'a mut String,
    input: &'a mut String,
}

impl HistoryHelper<'_> {
    fn go_back(&mut self) {
        let h = self.history.as_slice();
        if h.is_empty() {
            return;
        }
        let i = match *self.idx {
            None => {
                *self.saved = self.input.clone();
                0
            }
            Some(i) if i + 1 < h.len() => i + 1,
            _ => return,
        };
        *self.idx = Some(i);
        self.input.clear();
        self.input.push_str(&h[i]);
    }

    fn go_forward(&mut self) {
        match *self.idx {
            None => {}
            Some(0) => {
                *self.idx = None;
                self.input.clear();
                self.input.push_str(&self.saved);
            }
            Some(i) => {
                *self.idx = Some(i - 1);
                self.input.clear();
                self.input.push_str(&self.history[i - 1]);
            }
        }
    }
}
