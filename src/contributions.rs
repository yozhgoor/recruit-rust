use candidate::Contribution;
use yew::prelude::*;
use yewprint::{Intent, Tag, Text};

pub struct Contributions {
    props: ContributionProps,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct ContributionProps {
    pub contributions: &'static Contribution,
    pub highlighted_tech: Option<String>,
}

impl Component for Contributions {
    type Message = ();
    type Properties = ContributionProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Contributions { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let contrib_tags = self
            .props
            .contributions
            .techs
            .iter()
            .map(|x| {
                html! {
                    <Tag
                        class=classes!("tag")
                        intent={
                            match self.props.highlighted_tech.as_ref() {
                                Some(highlighted_tech) if highlighted_tech == x => Intent::Danger,
                                _ => Intent::Success
                            }
                        }
                    >
                        {x}
                    </Tag>
                }
            })
            .collect::<Html>();

        html! {
            <div class="candidate-alone-contribution">
                <div class="candidate-tag">
                    {contrib_tags}
                </div>
                    <a class="contribution-link" href={self.props.contributions.website}>
                        {self.props.contributions.project}
                    </a>
                    <Text class=classes!("contribution-description")>
                        {self.props.contributions.description}
                    </Text>
                <div class="separator">
                </div>
            </div>
        }
    }
}
