use patternfly_yew::*;
use yew::prelude::*;
use crate::queries::*;

pub struct QueryCard {
    table: Option<TableExample>,
	SearchInput: bool,
	SearchedTerm: String

}

#[derive(Clone, Debug)]
pub enum Msg {
    ConfirmedInput(String),
}

impl Component for QueryCard {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self { table: None, SearchInput: true, SearchedTerm: "".to_owned() }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ConfirmedInput(string) => {
				log::info!("Clicks: {}", string);
				if string.chars().last().unwrap() == '\n'{
				self.SearchInput = false;
				self.SearchedTerm = string;
				}
			},
        }
        //log::info!("Clicks: {}", self.value);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let title = html! {<>
            {""}
        </>};
		//let username_state = use_state(|| "".to_owned());
        html! {
            <div style="width: 75%;">
				<Card
					selectable=true
					selected=true
					title={title}
				>
				<div hidden={!self.SearchInput}>
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
				<p>{"Address Query: "}</p>
				<p>{self.SearchedTerm.clone()}</p>
				</div>
				<TableExample/>
				</Card>
            </div>
        }
    }
}
