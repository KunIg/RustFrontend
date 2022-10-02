use gloo::console::log;
use web_sys::{DragEvent};

use patternfly_yew::*;
use yew::prelude::*;

use chrono::Utc;

pub struct TableExample {
    model: SharedTableModel<ExampleEntry>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExampleEntry {
    pub entry: String,
}

impl TableRenderer for ExampleEntry {
    fn render(&self, column: ColumnIndex) -> Html {
        match column.index {
            0 => html! {<p style="text-color: black; text-align: center; background-color: white; border-radius: 6px;">{&self.entry}</p>},
            _ => html! {},
        }
    }
	/*
    fn render_details(&self) -> Vec<Span> {
        vec![Span::max(html! {
            <>
                { "So many details for " }{ &self.entry }
            </>
        })]
    }*/
}

pub enum Msg {
	Dragged(DragEvent),
	Dropped(DragEvent),
}

impl Component for TableExample {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        let model = vec![
            ExampleEntry {
                entry: "".into(),
            }
        ];

        Self {
            model: model.into(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
            Msg::Dragged(event) => {
                event.prevent_default();
				return false;
            },
            Msg::Dropped(event) => {
                event.prevent_default();
                let data_transfer = event.data_transfer().unwrap().get_data("text").unwrap();
				log!(format!("{:#?}",&data_transfer));
                self.model.push(ExampleEntry {
					entry: format!("{}",&data_transfer),
				})
            }
		}

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
		let link = ctx.link();
		let header = html_nested! {
			<TableHeader>
				<TableColumn label=""/>
			</TableHeader>
		};

		let target = html!{
			<div style = "float: right">
			<Button
			align={Align::End} 
			icon={Icon::Play} 
			variant={Variant::Link} 
			/>
			</div>
		};
        html! {
            <>
				<Table<SharedTableModel<ExampleEntry>>
					mode={TableMode::CompactNoBorders}
					header={header}
					entries={self.model.clone()}
				>
				</Table<SharedTableModel<ExampleEntry>>>
				<div style = "padding: 5px; float: left;border-width:3px; border-style:solid; border-color:#FF0000; text-align: center; border-radius: 6px;" class="drop-zone"
					ondragover={link.callback(|e| Msg::Dragged(e))}
					ondrop={link.callback(|e| Msg::Dropped(e))}>
                	<p>{ "drag queries here" }</p>
				</div>

				<Popover
				toggle_by_onclick=true
				target={target}
				>
				<div style="width:400%;">
					<img width = "800" src="img/pricevsgasprice.png" alt="price"/>
				</div>
				</Popover>

            </>
        }
    }
}
