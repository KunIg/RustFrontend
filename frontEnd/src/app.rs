use crate::queryCard::*;
use crate::full;
use crate::layouts::LayoutItem;
use web_sys::{HtmlDivElement};
use wasm_bindgen::JsCast;
use patternfly_yew::*;

use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::router::Render;

pub struct Model {
	not_rendered: bool
}

pub enum Msg {
	Clicked,
}

impl yew::Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: &Context<Self>) -> Self {
        Self {not_rendered : true}
    }

	fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
            Msg::Clicked => {
                self.not_rendered = false;
            },
            _ => {}
		}
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
		let drag = Callback::from(move |event: DragEvent| {
			event.data_transfer().unwrap()
			.set_data("text", &*event.target().unwrap().unchecked_into::<HtmlDivElement>().id());  
		});

        let sidebar = html_nested! {
            <PageSidebar>
				<div style="width:75%;">
				<Title level={Level::H1}>{""}
					<h1 style="color: white; margin: 14px; padding-left: 10px;">{"Time Range"}</h1>
				</Title>
				<Title level={Level::H4}>{""}
					<p style="text-color: black; margin: 14px; padding-left: 10px; background-color: white; border-radius: 6px;">{"Time - X Days"}
					<AppLauncher position={Position::Left}>
						<AppLauncherItem>
							<div id = "Time: 24 Hours" draggable="true" class="resizable" ondragstart={drag.clone()}>
								<p>{"24 Hours"}</p>
							</div>
						</AppLauncherItem>
						<Divider/>
						<AppLauncherItem>
							<div id = "Time: 30 Days" draggable="true" class="resizable" ondragstart={drag.clone()}>
								<p>{"30 Days"}</p>
							</div>
						</AppLauncherItem>
						<Divider/>
						<AppLauncherItem>
							<div id = "Time: 180 Days" draggable="true" class="resizable" ondragstart={drag.clone()}>
								<p>{"180 Days"}</p>
							</div>
						</AppLauncherItem>
						<Divider/>
						<AppLauncherItem>
							<div id = "Time: Custom" draggable="true" class="resizable" ondragstart={drag.clone()}>
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
				<Title level={Level::H4}>{""}
					<p style="text-color: black; margin: 14px; padding-left: 10px; background-color: white; border-radius: 6px">{"Top X Senders"}
					<AppLauncher position={Position::Left}>
						<AppLauncherItem>
							<div id = "Senders: Top 5" draggable="true" class="resizable" ondragstart={drag.clone()}>
								<p>{"Top 5"}</p>
							</div>
						</AppLauncherItem>
						<Divider/>
						<AppLauncherItem>
							<div id = "Senders: Top 10" draggable="true" class="resizable" ondragstart={drag.clone()}>
								<p>{"Top 10"}</p>
							</div>
						</AppLauncherItem>
						<Divider/>
						<AppLauncherItem>
							<div id = "Senders: Top 20" draggable="true" class="resizable" ondragstart={drag.clone()}>
								<p>{"Top 20"}</p>
							</div>
						</AppLauncherItem>
						<Divider/>
					</AppLauncher>
					</p>
				</Title>

				<Title level={Level::H4}>{""}
					<p style="text-color: black; margin: 14px; padding-left: 10px; background-color: white; border-radius: 6px;">{"Top Y Receivers"}
					<AppLauncher position={Position::Left}>
						<AppLauncherItem>
							<div id = "Receivers: Top 5" draggable="true" class="resizable" ondragstart={drag.clone()}>
								<p>{"Top 5"}</p>
							</div>
						</AppLauncherItem>
						<Divider/>
						<AppLauncherItem>
							<div id = "Receivers: Top 10" draggable="true" class="resizable" ondragstart={drag.clone()}>
								<p>{"Top 10"}</p>
							</div>
						</AppLauncherItem>
						<Divider/>
						<AppLauncherItem>
							<div id = "Receivers: Top 20" draggable="true" class="resizable" ondragstart={drag.clone()}>
								<p>{"Top 20"}</p>
							</div>
						</AppLauncherItem>
						<Divider/>
					</AppLauncher>
					</p>
				</Title>
				
				<Title level={Level::H4}>{""}
					<div style="cursor: pointer" id = "Total Fees Generated" draggable="true" class="resizable" ondragstart={drag.clone()}>
						<p style="text-color: black; margin: 14px; padding-left: 10px; background-color: white; border-radius: 6px;">{"Total Fees Generated"}</p>
					</div>
				</Title>

				<Title level={Level::H4}>{""}
					<div style="cursor: pointer" id = "Total Transactions" draggable="true" class="resizable" ondragstart={drag.clone()}>
						<p style="text-color: black; margin: 14px; padding-left: 10px; background-color: white; border-radius: 6px;">{"Total Transactions"}</p>
					</div>
				</Title>
				<Title level={Level::H4}>{""}
					<div style="cursor: pointer" id = "Fund Allocator" draggable="true" class="resizable" ondragstart={drag.clone()}>
						<p style="text-color: black; margin: 14px; padding-left: 10px; background-color: white; border-radius: 6px;">{"Fund Allocator"}</p>
					</div>
				</Title>	
				</div>			
            </PageSidebar>
        };

        let logo = html_nested! {
            <Logo src="img/logo.webp" alt="Logo" />
        };

        html! {
			<>
			<BackdropViewer/>
            <ToastViewer/>
            <Page
                logo={logo}
                sidebar={sidebar}
                >
				<Flex>
					<FlexItem>
					<LayoutItem>
						<QueryCard/>
					</LayoutItem>
					</FlexItem>
					<FlexItem>
					<LayoutItem>
					<div hidden={self.not_rendered}>
						<QueryCard/>
					</div>
					</LayoutItem>
					</FlexItem>
					<FlexItem>
					<LayoutItem>
					<Button label="Add Query" 
					align={Align::Start} 
					icon={Icon::PlusCircleIcon} 
					variant={Variant::Link} 
					onclick={ctx.link().callback(|_| Msg::Clicked)}/>
					</LayoutItem>
					</FlexItem>
				</Flex>
            </Page>
			</>
        }
    }
}