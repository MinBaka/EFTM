use iced::widget::row;
use iced::{Application, Command, Element, Length, Settings, Size, Theme};

// =============================================================================
// 1. 程序主入口
// =============================================================================
pub fn main() -> iced::Result {
    // 初始化应用程序，显式配置窗口初始大小和居中显示行为
    EftmApp::run(Settings {
        window: iced::window::Settings {
            size: Size::new(1200.0, 800.0),
            position: iced::window::Position::Centered,
            ..Default::default()
        },
        ..Default::default()
    })
}

// =============================================================================
// 2. 全局状态与消息定义
// =============================================================================

/// 侧边栏导航选项枚举
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

/// 应用程序核心状态
struct EftmApp {
    active_nav: NavItem,
    show_donate_banner: bool,
    show_notice_banner: bool,
}

/// 用户交互消息枚举
#[derive(Debug, Clone)]
enum Message {
    NavClicked(NavItem),
    HideDonateBanner,
    HideNoticeBanner,
    OpenMapSettings,
    DonateKoFi,
    ChangeTheme,
}

impl Application for EftmApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                active_nav: NavItem::MapView,
                show_donate_banner: true,
                show_notice_banner: true,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("EFTM - Escape From Tarkov Map (Rust)")
    }

    // =============================================================================
    // 3. 业务逻辑更新 (Update)
    // =============================================================================
    fn update(&mut self, message: Message) -> Command<Message> {
        // 捕获用户操作并更新界面状态
        match message {
            Message::NavClicked(item) => {
                self.active_nav = item;
            }
            Message::HideDonateBanner => {
                self.show_donate_banner = false;
            }
            Message::HideNoticeBanner => {
                self.show_notice_banner = false;
            }
            _ => {} // 对于尚未实现功能的消息（如占位按钮）暂时忽略
        }
        Command::none()
    }

    // =============================================================================
    // 4. 界面渲染 (View)
    // =============================================================================
    fn view(&self) -> Element<Message> {
        row![
            ui::sidebar::view(self.active_nav),
            ui::main_content::view(self.show_donate_banner, self.show_notice_banner),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
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
    /// 全局 UI 样式及调色板配置
    pub mod styles {
        use iced::Color;
        pub const SIDEBAR_BG: Color = Color::from_rgb(0.98, 0.98, 0.98);
        pub const CONTENT_BG: Color = Color::WHITE;
        pub const TEXT_DARK: Color = Color::from_rgb(0.2, 0.2, 0.2);
        pub const TEXT_LIGHT: Color = Color::from_rgb(0.6, 0.6, 0.6);
        pub const ACCENT_BLUE: Color = Color::from_rgb(0.1, 0.4, 0.8);
        pub const BANNER_RED_BG: Color = Color::from_rgb(1.0, 0.9, 0.9);
        pub const BANNER_RED_TEXT: Color = Color::from_rgb(0.8, 0.2, 0.2);
        pub const BANNER_BROWN_BG: Color = Color::from_rgb(0.3, 0.2, 0.1);
        pub const BANNER_BROWN_TEXT: Color = Color::from_rgb(0.9, 0.8, 0.7);
    }

    /// 左侧导航栏构建模块
    pub mod sidebar {
        use iced::widget::{button, column, container, row, scrollable, text, Space};
        use iced::{Alignment, Element, Length, Padding};
        use crate::{Message, NavItem};
        use super::styles;

        /// 渲染侧边栏主视图
        pub fn view(active_item: NavItem) -> Element<'static, Message> {
            column![
                // 顶部标题标识区
                column![
                    text("EFTM").size(24).style(styles::TEXT_DARK),
                    text("v1.0.0 - f5943bab6").size(12).style(styles::TEXT_LIGHT),
                ]
                .padding(20)
                .spacing(5),

                // 核心导航滚动区
                scrollable(column![
                    section_header("Main"),
                    nav_item("[M]", "Map View", NavItem::MapView, active_item),
                    nav_item("[H]", "Tactical HUD", NavItem::TacticalHud, active_item),
                    
                    Space::with_height(20),
                    
                    section_header("Tools"),
                    nav_item("[I]", "Item Manager", NavItem::ItemManager, active_item),
                    nav_item("[C]", "Loadout Catalogue", NavItem::LoadoutCatalogue, active_item),
                    
                    Space::with_height(20),
                    
                    section_header("Support"),
                    nav_item("[W]", "Wiki", NavItem::Wiki, active_item),
                    nav_item("[R]", "Roadmap", NavItem::Roadmap, active_item),
                    nav_item("[F]", "Feedback", NavItem::Feedback, active_item),
                ].padding(10)),

                Space::with_height(Length::Fill), // 弹性占位符以将底部区域推至最下

                // 底部用户设置区
                container(
                    button(
                        row![
                            text("[P]").style(styles::TEXT_DARK),
                            text("Anonymous").style(styles::TEXT_DARK),
                            text("^").style(styles::TEXT_LIGHT),
                        ]
                        .spacing(10)
                        .align_items(Alignment::Center)
                    )
                    .padding(10)
                    .style(iced::theme::Button::Text)
                )
                .padding(20),

                // 系统状态监控指示区
                container(
                    text("RAM: 50.2% <- 3.9% EFTM").size(12).style(styles::TEXT_LIGHT)
                )
                .padding(Padding { bottom: 10.0, left: 20.0, right: 20.0, top: 0.0 }),
            ]
            .width(Length::Fixed(260.0)) // 固定侧边栏宽度
            .height(Length::Fill)
            .into()
        }

        /// 生成导航组标题
        fn section_header(title: &str) -> Element<'static, Message> {
            // 将纯文本包裹在 container 中以应用内边距
            container(
                text(title)
                    .size(12)
                    .style(styles::TEXT_LIGHT)
            )
            .padding(Padding { bottom: 10.0, left: 10.0, right: 0.0, top: 10.0 })
            .into()
        }

        /// 生成具备状态反馈的导航项按钮
        fn nav_item(icon: &str, label: &str, item_type: NavItem, active_item: NavItem) -> Element<'static, Message> {
            let is_active = item_type == active_item;
            
            let content = row![
                text(icon).style(if is_active { styles::TEXT_DARK } else { styles::TEXT_LIGHT }).width(Length::Fixed(30.0)),
                text(label).style(styles::TEXT_DARK),
            ]
            .spacing(10)
            .align_items(Alignment::Center);

            let button_style = if is_active {
                iced::theme::Button::Secondary // 选中项高亮底色
            } else {
                iced::theme::Button::Text      // 未选中项透明底色
            };

            button(content)
                .on_press(Message::NavClicked(item_type))
                .padding(10)
                .width(Length::Fill)
                .style(button_style)
                .into()
        }
    }

    /// 右侧主内容区构建模块
    pub mod main_content {
        use iced::widget::{button, column, container, row, scrollable, text, Space};
        use iced::{Alignment, Color, Element, Length, Padding};
        use crate::Message;
        use super::styles;

        /// 渲染主内容数据视图
        pub fn view(show_donate: bool, show_notice: bool) -> Element<'static, Message> {
            container(
                scrollable(
                    column![
                        // 动态横幅区域 (通过全局状态控制渲染)
                        if show_donate {
                            donate_banner()
                        } else {
                            Element::from(Space::with_height(0))
                        },

                        if show_notice {
                            notice_banner()
                        } else {
                            Element::from(Space::with_height(0))
                        },

                        Space::with_height(20),

                        // 项目介绍模块
                        text_section("About", 
                            "EFTM is a project that aims to provide real-time maps and tactical overlay for Tarkov players, if you want to learn more then you can visit the github page or the wiki via the sidebar."
                        ),

                        Space::with_height(30),

                        // 运行统计模块
                        statistics_section(),

                        Space::with_height(30),

                        // 用户会话模块
                        column![
                            text("You are logged in.").size(14).style(styles::TEXT_DARK),
                            text("Welcome back, anonymous!").size(14).style(styles::TEXT_DARK),
                        ].spacing(5),

                        Space::with_height(30),

                        // 贡献者展示模块
                        text_section("Contributors", ""), 
                        
                        Space::with_height(Length::Fill),

                        // 底部版权及主题控制开关
                        row![
                            text("Powered by Iced (Rust GUI Framework)").size(12).style(styles::TEXT_LIGHT),
                            Space::with_width(Length::Fill),
                            button(text("[☀️/🌙]").style(styles::TEXT_LIGHT))
                                .on_press(Message::ChangeTheme)
                                .style(iced::theme::Button::Text),
                        ].align_items(Alignment::Center),
                    ]
                    .padding(30) 
                    .spacing(15) 
                )
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        }

        /// 渲染红色捐赠支持横幅
        fn donate_banner() -> Element<'static, Message> {
            container(
                column![
                    row![
                        text("Hey There!").size(18).style(styles::BANNER_RED_TEXT),
                        Space::with_width(Length::Fill),
                        button(text("[X]").style(styles::BANNER_RED_TEXT)) 
                            .on_press(Message::HideDonateBanner)
                            .style(iced::theme::Button::Text),
                    ].align_items(Alignment::Center),

                    text("I see you're using EFTM. Support the development by donating via Ko-Fi. Every donation is equal to weeks(!) of server hosting costs, we are eternally grateful for every bit of support!")
                        .size(14)
                        .style(styles::BANNER_RED_TEXT),

                    row![
                        button(text("Donate via Ko-Fi").style(styles::ACCENT_BLUE))
                            .on_press(Message::DonateKoFi)
                            .style(iced::theme::Button::Text),
                        button(text("Hide").style(styles::TEXT_DARK))
                            .on_press(Message::HideDonateBanner)
                            .style(iced::theme::Button::Text),
                    ].spacing(15),
                ]
                .spacing(10)
            )
            .padding(20)
            .width(Length::Fill)
            .into()
        }

        /// 渲染棕色系统通知横幅
        fn notice_banner() -> Element<'static, Message> {
            container(
                column![
                    row![
                        text("Notice!").size(18).style(styles::BANNER_BROWN_TEXT),
                        Space::with_width(Length::Fill),
                        button(text("[X]").style(styles::BANNER_BROWN_TEXT))
                            .on_press(Message::HideNoticeBanner)
                            .style(iced::theme::Button::Text),
                    ].align_items(Alignment::Center),

                    text("Map data for Customs is outdated. Please reinstall or update it in the Map settings.")
                        .size(14)
                        .style(styles::BANNER_BROWN_TEXT),

                    button(text("Open Map Settings").style(Color::WHITE))
                        .on_press(Message::OpenMapSettings)
                        .style(iced::theme::Button::Text),
                ]
                .spacing(10)
            )
            .padding(20)
            .width(Length::Fill)
            .into()
        }

        /// 渲染基础文本区块
        fn text_section(title: &str, body: &str) -> Element<'static, Message> {
            column![
                text(title).size(20).style(styles::TEXT_DARK),
                text(body).size(14).style(styles::TEXT_DARK),
            ]
            .spacing(10)
            .into()
        }

        /// 渲染统计数据区块
        fn statistics_section() -> Element<'static, Message> {
            column![
                text("Statistics").size(20).style(styles::TEXT_DARK),
                column![
                    stat_line("Tarkov players online:", "32 users"),
                    stat_line("Past 24 hours:", "706 unique users"),
                    stat_line("Your usage time:", "... over ... sessions"),
                ]
                .spacing(5)
                .padding(Padding { bottom: 0.0, left: 0.0, right: 0.0, top: 10.0 }),
            ]
            .spacing(10)
            .into()
        }

        /// 渲染单行统计数据字段
        fn stat_line(label: &str, value: &str) -> Element<'static, Message> {
            row![
                text(label).size(14).style(styles::TEXT_DARK),
                Space::with_width(10.0),
                text(value).size(14).style(styles::TEXT_DARK),
            ].into()
        }
    }
}