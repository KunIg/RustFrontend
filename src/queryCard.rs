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

impl Component for QueryCard {
    type Message = Msg;
    type Properties = ();

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
		//let username_state = use_state(|| "".to_owned());
		/*
		let input_changed = Callback::from(move |event: InputEvent| {
			log!(event.is_composing());
			log!(event.input_type());
			log!(event.data().unwrap());
			  
		});
		*/

        html! {
				<Card
					selectable=true
					selected=true
					title={title}
				>
				<div hidden={!self.SearchInput}>
				<input type="text" value={self.SearchedTerm.clone()} oninput={ctx.link().callback(move |event: InputEvent| Msg::OnInput(event))} />
				<Button label="Confirm" 
				align={Align::Start} 
				icon={Icon::CheckCircle} 
				variant={Variant::Link} 
				onclick={ctx.link().callback(|_| Msg::ConfirmedInput())}
				/>
				/*
				<Form>
					<FormGroupValidated<TextInput>
						label="Search Contract"
						required=true
						validator={Validator::from(|ctx: ValidationContext<String>| {
							if ctx.value.is_empty() {
								ValidationResult::error("Must not be empty")
							} else if ctx.value.chars().last().unwrap() == '\n' {
								ValidationResult::new(InputState::Success, ctx.value)
							} else {
								ValidationResult::default()
							}
						})}
					>
                    <TextInput
                        placeholder="Enter some text"
					/>
					</FormGroupValidated<TextInput>>
				</Form>	
				*/
					/*
					<Form>
						<TextInput
							onchange = {ctx.link().callback(|e| Msg::ConfirmedInput(e))}
							placeholder="Search Address"
						/>
						<Button label="Confirm" 
						align={Align::End} 
						icon={Icon::CheckCircle} 
						variant={Variant::Link} 
						//onclick={click.clone()}
						/>
					</Form>
					*/
				</div>
				<div hidden={self.SearchInput}>
				<p style="text-color: black; background-color: white; border-radius: 6px;">{"Query: "}</p>
				<p style="text-color: black; background-color: white; border-radius: 6px;">{self.SearchedTerm.clone()}</p>
				</div>
				<TableExample/>
				</Card>
        }
    }
}

/*
fn update(&mut self, msg: Self::Message) -> ShouldRender {
	match msg {
	  Msg::OnInput(value) => {
		let old_value = self.input_value.clone();
		self.input_value = "".to_string();
		self.link.send_self(Msg::SetInput(old_value);
		true
	  }
	  Msg::SetInput(value) => {
		self.input_value = value;
		true
	}
  }
  }
  */