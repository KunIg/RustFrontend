use crate::queryCard::*;
use crate::full;
use crate::layouts::LayoutItem;
use web_sys::{HtmlDivElement};
use wasm_bindgen::JsCast;
use patternfly_yew::*;

use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::router::Render;

pub struct Model {}

impl yew::Component for Model {
    type Message = ();
    type Properties = ();
    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
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
				<Title level={Level::H4}>{""}
					<p style="text-color: black; margin: 14px; padding-left: 10px; background-color: white; border-radius: 6px">{"Top X Senders"}
					<AppLauncher position={Position::Left}>
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

				<Title level={Level::H4}>{""}
					<p style="text-color: black; margin: 14px; padding-left: 10px; background-color: white; border-radius: 6px;">{"Top Y Receivers"}
					<AppLauncher position={Position::Left}>
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
				
				<Title level={Level::H4}>{""}
					<div id = "Total Fees Generated" draggable="true" class="resizable" ondragstart={drag.clone()}>
						<p style="text-color: black; margin: 14px; padding-left: 10px; background-color: white; border-radius: 6px;">{"Total Fees Generated"}</p>
					</div>
				</Title>

				<Title level={Level::H4}>{""}
					<div id = "Total Transactions" draggable="true" class="resizable" ondragstart={drag.clone()}>
						<p style="text-color: black; margin: 14px; padding-left: 10px; background-color: white; border-radius: 6px;">{"Total Transactions"}</p>
					</div>
				</Title>
				<Title level={Level::H4}>{""}
					<div id = "Fund Allocator" draggable="true" class="resizable" ondragstart={drag.clone()}>
						<p style="text-color: black; margin: 14px; padding-left: 10px; background-color: white; border-radius: 6px;">{"Fund Allocator"}</p>
					</div>
				</Title>					
			</div>
            </PageSidebar>
        };

        let logo = html_nested! {
            <Logo src="2MPlus_Logo.svg" alt="Logo" />
        };
		let click = Callback::from(|_| {
			
		});
        html! {
			<>
			<BackdropViewer/>
            <ToastViewer/>
            <Page
                logo={logo}
                sidebar={sidebar}
                >
				<Button label="Add Query" 
				align={Align::Start} 
				icon={Icon::PlusCircleIcon} 
				variant={Variant::Link} 
				onclick={click.clone()}/>
				<Flex>
					<FlexItem>
					<LayoutItem>
						<QueryCard/>
						<div id = "additionalBoxes"></div>
					</LayoutItem>
					</FlexItem>
				</Flex>
            </Page>
			</>
        }
    }
}