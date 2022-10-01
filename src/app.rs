use crate::components;
use crate::queryCard::*;
use crate::full;
use crate::index::*;
use crate::layouts;
use web_sys::{HtmlDivElement};
use wasm_bindgen::JsCast;

use patternfly_yew::*;

use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::router::Render;

pub struct Model {}

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum Component {
    #[to = "/alert"]
    Alert,
    #[to = "/applauncher"]
    AppLauncher,
    #[to = "/badge"]
    Badge,
    #[to = "/clipboard"]
    Clipboard,
    #[to = "/context_selector"]
    ContextSelector,
    #[to = "/dropdown"]
    Dropdown,
    #[to = "/empty"]
    EmptyState,
    #[to = "/form"]
    Form,
    #[to = "/label"]
    Label,
    #[to = "/modal"]
    Modal,
    #[to = "/popover"]
    Popover,
    #[to = "/select"]
    Select,
    #[to = "/slider"]
    Slider,
    #[to = "/switch"]
    Switch,
    #[to = "/tabs{*}"]
    Tabs(components::TabRoutes),
    #[to = "/table"]
    Table,
    #[to = "/text"]
    Text,
    #[to = "/title"]
    Title,
    #[to = "/tooltip"]
    Tooltip,
}

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum Layout {
    #[to = "/bullseye"]
    Bullseye,
    #[to = "/flex"]
    Flex,
    #[to = "/gallery"]
    Gallery,
    #[to = "/grid"]
    Grid,
    #[to = "/split"]
    Split,
    #[to = "/stack"]
    Stack,
}

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum FullPage {
    #[to = "/login"]
    Login,
}

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[to = "/components{*:rest}"]
    Component(Component),
    #[to = "/fullpage{*:rest}"]
    FullPageExample(FullPage),
    #[to = "/full{*:rest}"]
    FullPage(FullPage),
    #[to = "/layout{*:rest}"]
    Layout(Layout),
    #[to = "/QueryCard"]
    QueryCard,
    #[to = "/"]
    Index,
}

impl yew::Component for Model {
    type Message = ();
    type Properties = ();
    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <>
            <BackdropViewer/>
            <ToastViewer/>

            <Router<AppRoute, ()>
                redirect = {Router::redirect(|_|AppRoute::Index)}
                render = {Self::switch_main()}
            />
            </>
        }
    }
}

impl Model {
    fn switch_main() -> Render<AppRoute, ()> {
        Router::render(|switch: AppRoute| match switch {
            AppRoute::QueryCard => Self::page(html! {<QueryCard/>}),
            AppRoute::Index => Self::page(html! {<Index/>}),

            AppRoute::FullPageExample(FullPage::Login) => {
                Self::page(html! {<full::FullPageExample url="../../full/login"/>})
            }
            AppRoute::FullPage(FullPage::Login) => html! {<full::LoginPageExample/>},

            AppRoute::Layout(Layout::Bullseye) => Self::page(html! {<layouts::BullseyeExample/>}),
            AppRoute::Layout(Layout::Flex) => Self::page(html! {<layouts::FlexExample/>}),
            AppRoute::Layout(Layout::Gallery) => Self::page(html! {<layouts::GalleryExample/>}),
            AppRoute::Layout(Layout::Grid) => Self::page(html! {<layouts::GridExample/>}),
            AppRoute::Layout(Layout::Split) => Self::page(html! {<layouts::SplitExample/>}),
            AppRoute::Layout(Layout::Stack) => Self::page(html! {<layouts::StackExample/>}),

            AppRoute::Component(Component::Alert) => {
                Self::page(html! {<components::AlertExample/>})
            }
            AppRoute::Component(Component::AppLauncher) => {
                Self::page(html! {<components::AppLauncherExample/>})
            }
            AppRoute::Component(Component::Badge) => {
                Self::page(html! {<components::BadgeExample/>})
            }
            AppRoute::Component(Component::Clipboard) => {
                Self::page(html! {<components::ClipboardExample/>})
            }
            AppRoute::Component(Component::ContextSelector) => {
                Self::page(html! {<components::ContextSelectorExample/>})
            }
            AppRoute::Component(Component::Dropdown) => {
                Self::page(html! {<components::DropdownExample/>})
            }
            AppRoute::Component(Component::EmptyState) => {
                Self::page(html! {<components::EmptyStateExample/>})
            }
            AppRoute::Component(Component::Form) => Self::page(html! {<components::FormExample/>}),
            AppRoute::Component(Component::Label) => {
                Self::page(html! {<components::LabelExample/>})
            }
            AppRoute::Component(Component::Modal) => {
                Self::page(html! {<components::ModalExample/>})
            }
            AppRoute::Component(Component::Popover) => {
                Self::page(html! {<components::PopoverExample/>})
            }
            AppRoute::Component(Component::Select) => {
                Self::page(html! {<components::SelectExample/>})
            }
            AppRoute::Component(Component::Slider) => {
                Self::page(html! {<components::SliderExample/>})
            }
            AppRoute::Component(Component::Switch) => {
                Self::page(html! {<components::SwitchExample/>})
            }
            AppRoute::Component(Component::Table) => {
                Self::page(html! {<components::TableExample/>})
            }
            AppRoute::Component(Component::Tabs(current)) => {
                Self::page(html! {<components::TabsExample current={current}/>})
            }
            AppRoute::Component(Component::Text) => Self::page(html! {<components::TextExample/>}),
            AppRoute::Component(Component::Title) => {
                Self::page(html! {<components::TitleExample/>})
            }
            AppRoute::Component(Component::Tooltip) => {
                Self::page(html! {<components::TooltipExample/>})
            }
        })
    }

    fn page(html: Html) -> Html {

		let drag = Callback::from(move |event: DragEvent| {
			event.data_transfer().unwrap()
			.set_data("text", &*event.target().unwrap().unchecked_into::<HtmlDivElement>().id());  
		});

        let sidebar = html_nested! {
            <PageSidebar>
				<Nav>
					<NavRouterItem<AppRoute> to={AppRoute::FullPageExample(FullPage::Login)}>{"Login Page"}</NavRouterItem<AppRoute>>
					//<NavItem external=true to={AppRoute::FullPageExample(FullPage::Login)}>{"PatternFly Yew"}</NavItem>
				</Nav>
				<Title level={Level::H1}>{""}
					<h1 style="color: white; margin: 14px; padding-left: 10px;">{"Time Range"}</h1>
				</Title>
				<Title level={Level::H2}>{""}
					<p style="text-color: black; margin: 14px; padding-left: 10px; background-color: white; border-radius: 6px;">{"Time - X Days"}
					<AppLauncher>
						<AppLauncherItem>
							<div id = "24 Hours" draggable="true" class="resizable" ondragstart={drag.clone()}>
								<p>{"24 Hours"}</p>
							</div>
						</AppLauncherItem>
						<Divider/>
						<AppLauncherItem>
							<div id = "30 Days" draggable="true" class="resizable" ondragstart={drag.clone()}>
								<p>{"30 Days"}</p>
							</div>
						</AppLauncherItem>
						<Divider/>
						<AppLauncherItem>
							<div id = "180 Days" draggable="true" class="resizable" ondragstart={drag.clone()}>
								<p>{"180 Days"}</p>
							</div>
						</AppLauncherItem>
						<Divider/>
						<AppLauncherItem>
							<div id = "Custom" draggable="true" class="resizable" ondragstart={drag.clone()}>
								<p>{"Custom"}</p>
							</div>
						</AppLauncherItem>
						<Divider/>
					</AppLauncher>
					</p>
				</Title>
				<Title level={Level::H1}>{""}
					<h1 style="color: white; margin: 14px; padding-left: 10px;">{"Address Queries"}</h1>
				</Title>
				<Title level={Level::H2}>{""}
					<p style="text-color: black; margin: 14px; padding-left: 10px; background-color: white; border-radius: 6px;">{"Top X Senders"}
					<AppLauncher>
						<AppLauncherItem>
							<div id = "Top 5" draggable="true" class="resizable" ondragstart={drag.clone()}>
								<p>{"Top 5"}</p>
							</div>
						</AppLauncherItem>
						<Divider/>
						<AppLauncherItem>
							<div id = "Top 10" draggable="true" class="resizable" ondragstart={drag.clone()}>
								<p>{"Top 10"}</p>
							</div>
						</AppLauncherItem>
						<Divider/>
						<AppLauncherItem>
							<div id = "Top 20" draggable="true" class="resizable" ondragstart={drag.clone()}>
								<p>{"Top 20"}</p>
							</div>
						</AppLauncherItem>
						<Divider/>
					</AppLauncher>
					</p>
				</Title>

				<Title level={Level::H2}>{""}
					<p style="text-color: black; margin: 14px; padding-left: 10px; background-color: white; border-radius: 6px;">{"Top Y Receivers"}
					<AppLauncher>
						<AppLauncherItem>
							<div id = "Top 5" draggable="true" class="resizable" ondragstart={drag.clone()}>
								<p>{"Top 5"}</p>
							</div>
						</AppLauncherItem>
						<Divider/>
						<AppLauncherItem>
							<div id = "Top 10" draggable="true" class="resizable" ondragstart={drag.clone()}>
								<p>{"Top 10"}</p>
							</div>
						</AppLauncherItem>
						<Divider/>
						<AppLauncherItem>
							<div id = "Top 20" draggable="true" class="resizable" ondragstart={drag.clone()}>
								<p>{"Top 20"}</p>
							</div>
						</AppLauncherItem>
						<Divider/>
					</AppLauncher>
					</p>
				</Title>
				
				<Title level={Level::H2}>{""}
					<div id = "Total Fees Generated" draggable="true" class="resizable" ondragstart={drag.clone()}>
						<p style="text-color: black; margin: 14px; padding-left: 10px; background-color: white; border-radius: 6px;">{"Total Fees Generated"}</p>
					</div>
				</Title>

				<Title level={Level::H2}>{""}
					<div id = "Total Transactions" draggable="true" class="resizable" ondragstart={drag.clone()}>
						<p style="text-color: black; margin: 14px; padding-left: 10px; background-color: white; border-radius: 6px;">{"Total Transactions"}</p>
					</div>
				</Title>
				<Title level={Level::H2}>{""}
					<div id = "Fund Allocator" draggable="true" class="resizable" ondragstart={drag.clone()}>
						<p style="text-color: black; margin: 14px; padding-left: 10px; background-color: white; border-radius: 6px;">{"Fund Allocator"}</p>
					</div>
				</Title>					
				/*
                <Nav>
                    <NavRouterExpandable<AppRoute> title="Basics">
                        <NavRouterItem<AppRoute> to={AppRoute::Index}>{"Index"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::QueryCard}>{"QueryCard"}</NavRouterItem<AppRoute>>
                        <NavItem external=true to="https://github.com/ctron/patternfly-yew">{"PatternFly Yew"}</NavItem>
                    </NavRouterExpandable<AppRoute>>
                    <NavRouterExpandable<AppRoute> title="Components">
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Alert)}>{"Alert"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::AppLauncher)}>{"AppLauncher"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Badge)}>{"Badge"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Clipboard)}>{"Clipboard"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::ContextSelector)}>{"ContextSelector"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Dropdown)}>{"Dropdown"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::EmptyState)}>{"Empty state"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Form)}>{"Form"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Label)}>{"Label"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Modal)}>{"Modal"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Popover)}>{"Popover"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Select)}>{"Select"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Slider)}>{"Slider"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Switch)}>{"Switch"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Table)}>{"Table"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Tabs(components::TabRoutes::Foo))}>{"Tabs"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Text)}>{"Text"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Title)}>{"Title"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Tooltip)}>{"Tooltip"}</NavRouterItem<AppRoute>>
                    </NavRouterExpandable<AppRoute>>
                    <NavRouterExpandable<AppRoute> title="Layouts">
                        <NavRouterItem<AppRoute> to={AppRoute::Layout(Layout::Bullseye)}>{"Bullseye"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Layout(Layout::Flex)}>{"Flex"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Layout(Layout::Gallery)}>{"Gallery"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Layout(Layout::Grid)}>{"Grid"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Layout(Layout::Split)}>{"Split"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Layout(Layout::Stack)}>{"Stack"}</NavRouterItem<AppRoute>>
                    </NavRouterExpandable<AppRoute>>
                    <NavRouterExpandable<AppRoute> title="Full Page">
                        <NavRouterItem<AppRoute> to={AppRoute::FullPageExample(FullPage::Login)}>{"Login Page"}</NavRouterItem<AppRoute>>
                    </NavRouterExpandable<AppRoute>>
                </Nav>
				*/
            </PageSidebar>
        };

        let logo = html_nested! {
            <Logo src="2MPlus_Logo.svg" alt="Logo" />
        };

        html! {
            <Page
                logo={logo}
                sidebar={sidebar}
                >
				<QueryCard/>
                { html }
            </Page>
        }
    }
}
