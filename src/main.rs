use iced::widget::row;
use iced::{Application, Command, Element, Length, Settings, Size, Theme};

// =============================================================================
// 1. 程序主入口
// =============================================================================
pub fn main() -> iced::Result {
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
}

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
            _ => {}
        }
        Command::none()
    }

    // =============================================================================
    // 4. 界面渲染 (View) - 根据状态切换右侧内容
    // =============================================================================
    fn view(&self) -> Element<Message> {
        // 核心路由逻辑：根据当前选中的侧边栏项，决定右侧渲染哪个模块
        let content_area = match self.active_nav {
            // 如果选中了地图视图，则只渲染地图组件，隐藏原本的文字信息
            NavItem::MapView => ui::map_content::view(),
            // 否则渲染包含统计信息和横幅的默认主页
            _ => ui::main_content::view(self.show_donate_banner, self.show_notice_banner),
        };

        row![
            ui::sidebar::view(self.active_nav),
            content_area,
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
    use iced::{Background, Border, Color, Shadow};

    /// 全局 UI 样式及调色板配置
    pub mod styles {
        use super::*;
        pub const TEXT_DARK: Color = Color::from_rgb(0.2, 0.2, 0.2);
        pub const TEXT_LIGHT: Color = Color::from_rgb(0.6, 0.6, 0.6);
        pub const ACCENT_BLUE: Color = Color::from_rgb(0.1, 0.4, 0.8);
        pub const BANNER_RED_TEXT: Color = Color::from_rgb(0.8, 0.2, 0.2);
        pub const BANNER_BROWN_TEXT: Color = Color::from_rgb(0.9, 0.8, 0.7);

        /// 侧边栏左下角 RAM 监控框的样式定义
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

        /// 地图渲染区域的占位背景样式 (深色战术风格)
        pub fn map_bg_style(_theme: &iced::Theme) -> iced::widget::container::Appearance {
            iced::widget::container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.15, 0.15, 0.15))),
                ..Default::default()
            }
        }
    }

    /// 左侧导航栏构建模块
    pub mod sidebar {
        use iced::widget::{button, column, container, row, scrollable, text, Space};
        use iced::{Alignment, Element, Length, Padding};
        use crate::{Message, NavItem};
        use super::styles;

        pub fn view(active_item: NavItem) -> Element<'static, Message> {
            column![
                column![
                    text("EFTM").size(24).style(styles::TEXT_DARK),
                    text("v1.0.0 - f5943bab6").size(12).style(styles::TEXT_LIGHT),
                ]
                .padding(20)
                .spacing(5),

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

                Space::with_height(Length::Fill),

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

                // 优化：将 RAM 信息包裹在带有自定义边框样式的容器中
                container(
                    text("RAM: 50.2% <- 3.9% EFTM").size(12).style(styles::TEXT_LIGHT)
                )
                .padding(Padding { bottom: 8.0, left: 12.0, right: 12.0, top: 8.0 })
                .style(styles::ram_box_style) // 应用边框和白色背景
                .width(Length::Fill),
            ]
            .width(Length::Fixed(260.0))
            .height(Length::Fill)
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
            ]
            .spacing(10)
            .align_items(Alignment::Center);

            let button_style = if is_active {
                iced::theme::Button::Secondary
            } else {
                iced::theme::Button::Text
            };

            button(content)
                .on_press(Message::NavClicked(item_type))
                .padding(10)
                .width(Length::Fill)
                .style(button_style)
                .into()
        }
    }

    /// 地图专用渲染模块 (当左侧选中 Map View 时显示)
    pub mod map_content {
        use iced::widget::{container, text};
        use iced::{Color, Element, Length};
        use crate::Message;
        use super::styles;

        /// 渲染全屏的地图占位视图
        pub fn view() -> Element<'static, Message> {
            container(
                text("[ Tarkov Map Rendering Engine ]")
                    .size(24)
                    .style(Color::from_rgb(0.4, 0.4, 0.4)) // 占位文字设为暗灰色
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(styles::map_bg_style) // 应用深色背景样式
            .into()
        }
    }

    /// 常规信息展示模块 (其他菜单项的默认视图)
    pub mod main_content {
        use iced::widget::{button, column, container, row, scrollable, text, Space};
        use iced::{Alignment, Color, Element, Length, Padding};
        use crate::Message;
        use super::styles;

        pub fn view(show_donate: bool, show_notice: bool) -> Element<'static, Message> {
            container(
                scrollable(
                    column![
                        if show_donate { donate_banner() } else { Element::from(Space::with_height(0)) },
                        if show_notice { notice_banner() } else { Element::from(Space::with_height(0)) },

                        Space::with_height(20),
                        text_section("About", "EFTM is a project that aims to provide real-time maps and tactical overlay for Tarkov players..."),
                        Space::with_height(30),
                        statistics_section(),
                        Space::with_height(30),
                        
                        column![
                            text("You are logged in.").size(14).style(styles::TEXT_DARK),
                            text("Welcome back, anonymous!").size(14).style(styles::TEXT_DARK),
                        ].spacing(5),

                        Space::with_height(30),
                        text_section("Contributors", ""), 
                        Space::with_height(Length::Fill),

                        row![
                            text("Powered by Iced").size(12).style(styles::TEXT_LIGHT),
                            Space::with_width(Length::Fill),
                            button(text("[☀️/🌙]").style(styles::TEXT_LIGHT)).on_press(Message::ChangeTheme).style(iced::theme::Button::Text),
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

        fn donate_banner() -> Element<'static, Message> {
            // 省略重复逻辑，保持精简
            container(column![
                row![
                    text("Hey There!").size(18).style(styles::BANNER_RED_TEXT),
                    Space::with_width(Length::Fill),
                    button(text("[X]").style(styles::BANNER_RED_TEXT)).on_press(Message::HideDonateBanner).style(iced::theme::Button::Text),
                ].align_items(Alignment::Center),
                text("Support the development by donating via Ko-Fi...").size(14).style(styles::BANNER_RED_TEXT),
            ].spacing(10)).padding(20).width(Length::Fill).into()
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
            ].spacing(10)).padding(20).width(Length::Fill).into()
        }

        fn text_section(title: &str, body: &str) -> Element<'static, Message> {
            column![text(title).size(20).style(styles::TEXT_DARK), text(body).size(14).style(styles::TEXT_DARK)].spacing(10).into()
        }

        fn statistics_section() -> Element<'static, Message> {
            column![
                text("Statistics").size(20).style(styles::TEXT_DARK),
                column![
                    stat_line("Tarkov players online:", "32 users"),
                    stat_line("Past 24 hours:", "706 unique users"),
                ].spacing(5).padding(Padding { bottom: 0.0, left: 0.0, right: 0.0, top: 10.0 }),
            ].spacing(10).into()
        }

        fn stat_line(label: &str, value: &str) -> Element<'static, Message> {
            row![text(label).size(14).style(styles::TEXT_DARK), Space::with_width(10.0), text(value).size(14).style(styles::TEXT_DARK)].into()
        }
    }
}