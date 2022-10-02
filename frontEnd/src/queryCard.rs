use patternfly_yew::*;
use yew::prelude::*;
use crate::queries::*;
use gloo::console::log;
use web_sys::InputEvent;
use wasm_bindgen::JsCast;

pub struct QueryCard {
    table: Option<TableExample>,
	SearchInput: bool,
	SearchedTerm: String

}

#[derive(Clone, Debug)]
pub enum Msg {
    ConfirmedInput(),
	OnInput(InputEvent)
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
	#[prop_or_default]
    pub instance: u32,
}

impl Component for QueryCard {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self { table: None, SearchInput: true, SearchedTerm: "".to_owned() }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ConfirmedInput() => {
				self.SearchInput = false;
				
			},
			Msg::OnInput(event) => {
				let input_type = event.input_type();
				if input_type =="insertText"{
					self.SearchedTerm.push(event.data().unwrap().chars().collect::<Vec<char>>()[0]);
				} else if input_type == "deleteContentBackward" {
					self.SearchedTerm.pop();
				} else {

				}
			}
        }
        //log::info!("Clicks: {}", self.value);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let title = html! {<>
            {""}
        </>};

        html! {
				<Card
					selectable=true
					selected=true
					title={title}
				>
				<div hidden={!self.SearchInput}>
					<input type="text" placeholder="Search Query" value={self.SearchedTerm.clone()} oninput={ctx.link().callback(move |event: InputEvent| Msg::OnInput(event))} />
					<Button label="Confirm" 
					align={Align::Start} 
					icon={Icon::CheckCircle} 
					variant={Variant::Link} 
					onclick={ctx.link().callback(|_| Msg::ConfirmedInput())}
					/>
				</div>
				<div hidden={self.SearchInput}>
					<p style="text-color: black; background-color: white; border-radius: 6px;">{"Query: "}</p>
					<p style="text-color: black; background-color: white; border-radius: 6px;">{self.SearchedTerm.clone()}</p>
				</div>
				<TableExample instance={ctx.props().instance}/>
				</Card>
        }
    }
}
