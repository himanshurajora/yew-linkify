use linkify::{LinkFinder, LinkKind};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: String,
}

#[function_component]
pub fn Linkify(props: &Props) -> Html {
    let finder = LinkFinder::new();
    let text = props.text.as_str();
    let spans = finder.spans(text);

    html! {
        <p>
        {
            for spans.into_iter().map(move |current| {

            match current.kind() {

                Some(LinkKind::Email) => {
                    let mut mail_to_link: String = "mailto:".to_owned();
                    let current_string: &str = current.as_str();

                    mail_to_link.push_str(current_string);
                    return html!{<a href={mail_to_link}>{current.as_str()}</a>}
                }

                Some(LinkKind::Url) => {
                    // TODO: I don't know why I have to do this, will learn more rust and improve it later
                    let mut plain_link: String = "".to_owned();
                    let current_string: &str = current.as_str();
                    plain_link.push_str(current_string);

                    return html!{<a href={plain_link}>{current.as_str()}</a>}
                }
                _ => {
                return html!{
                    <span>{current.as_str()}</span>
                }
            }
            }

            })
        }
        </p>
    }
}

#[cfg(test)]
mod tests {
    use linkify::{LinkFinder, LinkKind};

    #[test]
    fn test_linkify_email() {
        let finder = LinkFinder::new();
        let text = "Contact: test@example.com";
        let spans: Vec<_> = finder.spans(text).collect();
        assert!(spans.iter().any(|s| s.kind() == Some(&LinkKind::Email)));
    }

    #[test]
    fn test_linkify_url() {
        let finder = LinkFinder::new();
        let text = "Visit https://example.com!";
        let spans: Vec<_> = finder.spans(text).collect();
        assert!(spans.iter().any(|s| s.kind() == Some(&LinkKind::Url)));
    }
}
