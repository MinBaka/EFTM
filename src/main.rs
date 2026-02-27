use iced::widget::{button, column, container, row, text, Space};
use iced::{Alignment, Application, Color, Command, Element, Length, Padding, Settings, Size, Subscription, Theme, window};
use sysinfo::System;

// =============================================================================
// 1. 程序主入口
// =============================================================================
pub fn main() -> iced::Result {
    // 初始化无边框应用程序
    EftmApp::run(Settings {
        window: iced::window::Settings {
            size: Size::new(1200.0, 800.0),
            position: iced::window::Position::Centered,
            decorations: false, // 禁用系统默认标题栏
            ..Default::default()
        },
        ..Default::default()
    })
}

// =============================================================================
// 2. 全局状态与消息定义
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NavItem {
    MapView,
    TacticalHud,
    ItemManager,
    LoadoutCatalogue,
    Wiki,
    Roadmap,
    Feedback,
}

struct EftmApp {
    active_nav: NavItem,
    show_donate_banner: bool,
    show_notice_banner: bool,
    sys: System,
    ram_display: String,
}

#[derive(Debug, Clone)]
enum Message {
    NavClicked(NavItem),
    HideDonateBanner,
    HideNoticeBanner,
    OpenMapSettings,
    DonateKoFi,
    ChangeTheme,
    Exit,
    Tick,
    TitleBarDragged,
}

impl Application for EftmApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut sys = System::new_all();
        sys.refresh_memory();
        
        (
            Self {
                active_nav: NavItem::MapView,
                show_donate_banner: true,
                show_notice_banner: true,
                sys,
                ram_display: String::from("RAM: --% <- -- MB EFTM"),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("EFTM")
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_secs(1)).map(|_| Message::Tick)
    }

    // =============================================================================
    // 3. 业务逻辑更新 (Update)
    // =============================================================================
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NavClicked(item) => {
                self.active_nav = item;
                Command::none()
            }
            Message::HideDonateBanner => {
                self.show_donate_banner = false;
                Command::none()
            }
            Message::HideNoticeBanner => {
                self.show_notice_banner = false;
                Command::none()
            }
            Message::Exit => {
                // 安全退出进程
                std::process::exit(0);
            }
            Message::TitleBarDragged => {
                // 触发底层系统调用，接管鼠标拖拽窗口事件
                window::drag(window::Id::MAIN)
            }
            Message::Tick => {
                self.sys.refresh_memory();
                self.sys.refresh_processes();

                let total_mem = self.sys.total_memory();
                let used_mem = self.sys.used_memory();
                let mem_percent = if total_mem > 0 {
                    (used_mem as f64 / total_mem as f64) * 100.0
                } else {
                    0.0
                };

                let mut app_mem_mb = 0.0;
                if let Ok(pid) = sysinfo::get_current_pid() {
                    if let Some(process) = self.sys.process(pid) {
                        app_mem_mb = process.memory() as f64 / 1024.0;
                    }
                }

                self.ram_display = format!("RAM: {:.1}% <- {:.1} MB EFTM", mem_percent, app_mem_mb);
                Command::none()
            }
            _ => Command::none(),
        }
    }

    // =============================================================================
    // 4. 界面渲染 (View)
    // =============================================================================
    fn view(&self) -> Element<Message> {
        let content_area = match self.active_nav {
            NavItem::MapView => ui::map_content::view(),
            _ => ui::main_content::view(self.show_donate_banner, self.show_notice_banner),
        };

        // 构建自定义拖拽标题栏：不再使用闭包，而是通过函数指针调用样式
        let title_bar = container(
            row![
                // 拖拽热区：使用一个填满剩余空间的不可见按钮作为拖拽把手
                button(
                    row![
                        Space::with_width(20.0),
                        text("EFTM - Tactical Map Overlay")
                            .size(12)
                            .style(Color::from_rgb(0.5, 0.5, 0.5)),
                        Space::with_width(Length::Fill),
                    ].align_items(Alignment::Center)
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .style(iced::theme::Button::Text)
                .on_press(Message::TitleBarDragged),

                // 右侧关闭按钮
                button(text("  X  ").style(Color::WHITE))
                    .style(iced::theme::Button::Destructive)
                    .padding(Padding::from([6.0, 12.0]))
                    .on_press(Message::Exit)
            ]
            .width(Length::Fill)
            .height(Length::Fixed(32.0))
        )
        .style(ui::styles::title_bar_style); // 使用显式函数引用避免生命周期推断错误

        let main_layout = row![
            ui::sidebar::view(self.active_nav, &self.ram_display),
            content_area,
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        column![
            title_bar,
            main_layout
        ]
        .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Light
    }
}

// =============================================================================
// 5. 模块化 UI 构建器
// =============================================================================
mod ui {
    use iced::{Background, Border, Color, Shadow};

    pub mod styles {
        use super::*;
        pub const TEXT_DARK: Color = Color::from_rgb(0.2, 0.2, 0.2);
        pub const TEXT_LIGHT: Color = Color::from_rgb(0.6, 0.6, 0.6);
        pub const BANNER_RED_TEXT: Color = Color::from_rgb(0.8, 0.2, 0.2);
        pub const BANNER_BROWN_TEXT: Color = Color::from_rgb(0.9, 0.8, 0.7);

        /// 标题栏背景样式函数
        pub fn title_bar_style(_theme: &iced::Theme) -> iced::widget::container::Appearance {
            iced::widget::container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
                ..Default::default()
            }
        }

        /// 侧边栏整体背景样式函数
        pub fn sidebar_style(_theme: &iced::Theme) -> iced::widget::container::Appearance {
            iced::widget::container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.98, 0.98, 0.98))),
                ..Default::default()
            }
        }

        /// RAM 监控框样式
        pub fn ram_box_style(_theme: &iced::Theme) -> iced::widget::container::Appearance {
            iced::widget::container::Appearance {
                background: Some(Background::Color(Color::WHITE)),
                border: Border {
                    color: Color::from_rgb(0.85, 0.85, 0.85),
                    width: 1.0,
                    radius: 6.0.into(),
                },
                text_color: None,
                shadow: Shadow::default(),
            }
        }

        /// 地图区占位背景样式
        pub fn map_bg_style(_theme: &iced::Theme) -> iced::widget::container::Appearance {
            iced::widget::container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.15, 0.15, 0.15))),
                ..Default::default()
            }
        }
    }

    pub mod sidebar {
        use iced::widget::{button, column, container, row, scrollable, text, Space};
        use iced::{Alignment, Element, Length, Padding};
        use crate::{Message, NavItem};
        use super::styles;

        pub fn view<'a>(active_item: NavItem, ram_display: &'a str) -> Element<'a, Message> {
            container(
                column![
                    column![
                        text("EFTM").size(24).style(styles::TEXT_DARK),
                        text("v1.0.0 - f5943bab6").size(12).style(styles::TEXT_LIGHT),
                    ]
                    .padding(Padding { bottom: 20.0, left: 20.0, right: 20.0, top: 10.0 })
                    .spacing(5.0),

                    scrollable(column![
                        section_header("Main"),
                        nav_item("[M]", "Map View", NavItem::MapView, active_item),
                        nav_item("[H]", "Tactical HUD", NavItem::TacticalHud, active_item),
                        Space::with_height(20.0),
                        section_header("Tools"),
                        nav_item("[I]", "Item Manager", NavItem::ItemManager, active_item),
                        nav_item("[C]", "Loadout Catalogue", NavItem::LoadoutCatalogue, active_item),
                        Space::with_height(20.0),
                        section_header("Support"),
                        nav_item("[W]", "Wiki", NavItem::Wiki, active_item),
                        nav_item("[R]", "Roadmap", NavItem::Roadmap, active_item),
                        nav_item("[F]", "Feedback", NavItem::Feedback, active_item),
                    ].padding(10.0)),

                    Space::with_height(Length::Fill),

                    container(
                        button(
                            row![
                                text("[P]").style(styles::TEXT_DARK),
                                text("Anonymous").style(styles::TEXT_DARK),
                                text("^").style(styles::TEXT_LIGHT),
                            ].spacing(10.0).align_items(Alignment::Center)
                        ).padding(10.0).style(iced::theme::Button::Text)
                    ).padding(20.0),

                    container(
                        text(ram_display).size(12).style(styles::TEXT_LIGHT)
                    )
                    .padding(Padding { bottom: 8.0, left: 12.0, right: 12.0, top: 8.0 })
                    .style(styles::ram_box_style)
                    .width(Length::Fill),
                ]
                .width(Length::Fixed(260.0))
                .height(Length::Fill)
            )
            .style(styles::sidebar_style) // 使用显式函数引用
            .into()
        }

        fn section_header(title: &str) -> Element<'static, Message> {
            container(text(title).size(12).style(styles::TEXT_LIGHT))
            .padding(Padding { bottom: 10.0, left: 10.0, right: 0.0, top: 10.0 })
            .into()
        }

        fn nav_item(icon: &str, label: &str, item_type: NavItem, active_item: NavItem) -> Element<'static, Message> {
            let is_active = item_type == active_item;
            let content = row![
                text(icon).style(if is_active { styles::TEXT_DARK } else { styles::TEXT_LIGHT }).width(Length::Fixed(30.0)),
                text(label).style(styles::TEXT_DARK),
            ].spacing(10.0).align_items(Alignment::Center);

            button(content)
                .on_press(Message::NavClicked(item_type))
                .padding(10.0)
                .width(Length::Fill)
                .style(if is_active { iced::theme::Button::Secondary } else { iced::theme::Button::Text })
                .into()
        }
    }

    pub mod map_content {
        use iced::widget::{container, text};
        use iced::{Color, Element, Length};
        use crate::Message;
        use super::styles;

        pub fn view() -> Element<'static, Message> {
            container(
                text("[ Tarkov Map Rendering Engine ]")
                    .size(24)
                    .style(Color::from_rgb(0.4, 0.4, 0.4))
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(styles::map_bg_style)
            .into()
        }
    }

    pub mod main_content {
        use iced::widget::{button, column, container, row, scrollable, text, Space};
        use iced::{Alignment, Color, Element, Length, Padding};
        use crate::Message;
        use super::styles;

        pub fn view(show_donate: bool, show_notice: bool) -> Element<'static, Message> {
            container(
                scrollable(
                    column![
                        if show_donate { donate_banner() } else { Element::from(Space::with_height(0.0)) },
                        if show_notice { notice_banner() } else { Element::from(Space::with_height(0.0)) },
                        Space::with_height(20.0),
                        text_section("About", "EFTM is a project that aims to provide real-time maps and tactical overlay for Tarkov players..."),
                        Space::with_height(30.0),
                        statistics_section(),
                        Space::with_height(30.0),
                        column![
                            text("You are logged in.").size(14).style(styles::TEXT_DARK),
                            text("Welcome back, anonymous!").size(14).style(styles::TEXT_DARK),
                        ].spacing(5.0),
                        Space::with_height(30.0),
                        text_section("Contributors", ""), 
                        Space::with_height(Length::Fill),
                        row![
                            text("Powered by Iced").size(12).style(styles::TEXT_LIGHT),
                            Space::with_width(Length::Fill),
                            button(text("[☀️/🌙]").style(styles::TEXT_LIGHT)).on_press(Message::ChangeTheme).style(iced::theme::Button::Text),
                        ].align_items(Alignment::Center),
                    ]
                    .padding(30.0) 
                    .spacing(15.0) 
                )
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        }

        fn donate_banner() -> Element<'static, Message> {
            container(column![
                row![
                    text("Hey There!").size(18).style(styles::BANNER_RED_TEXT),
                    Space::with_width(Length::Fill),
                    button(text("[X]").style(styles::BANNER_RED_TEXT)).on_press(Message::HideDonateBanner).style(iced::theme::Button::Text),
                ].align_items(Alignment::Center),
                text("Support the development by donating via Ko-Fi...").size(14).style(styles::BANNER_RED_TEXT),
            ].spacing(10.0)).padding(20.0).width(Length::Fill).into()
        }

        fn notice_banner() -> Element<'static, Message> {
            container(column![
                row![
                    text("Notice!").size(18).style(styles::BANNER_BROWN_TEXT),
                    Space::with_width(Length::Fill),
                    button(text("[X]").style(styles::BANNER_BROWN_TEXT)).on_press(Message::HideNoticeBanner).style(iced::theme::Button::Text),
                ].align_items(Alignment::Center),
                text("Map data for Customs is outdated...").size(14).style(styles::BANNER_BROWN_TEXT),
                button(text("Open Map Settings").style(Color::WHITE)).on_press(Message::OpenMapSettings).style(iced::theme::Button::Text),
            ].spacing(10.0)).padding(20.0).width(Length::Fill).into()
        }

        fn text_section(title: &str, body: &str) -> Element<'static, Message> {
            column![text(title).size(20).style(styles::TEXT_DARK), text(body).size(14).style(styles::TEXT_DARK)].spacing(10.0).into()
        }

        fn statistics_section() -> Element<'static, Message> {
            column![
                text("Statistics").size(20).style(styles::TEXT_DARK),
                column![
                    stat_line("Tarkov players online:", "32 users"),
                    stat_line("Past 24 hours:", "706 unique users"),
                ].spacing(5.0).padding(Padding { bottom: 0.0, left: 0.0, right: 0.0, top: 10.0 }),
            ].spacing(10.0).into()
        }

        fn stat_line(label: &str, value: &str) -> Element<'static, Message> {
            row![text(label).size(14).style(styles::TEXT_DARK), Space::with_width(10.0), text(value).size(14).style(styles::TEXT_DARK)].into()
        }
    }
}