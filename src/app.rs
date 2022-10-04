use enum_display::EnumDisplay;
use gloo_timers::callback::Interval;
use yew::{function_component, html::Scope, prelude::*};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Play,
    Guess,
}

impl Default for Phase {
    fn default() -> Self {
        Self::Play
    }
}

#[derive(Clone, Copy, PartialEq, Eq, EnumDisplay, Debug)]
#[enum_display(case = "Camel")]
pub enum Color {
    Red,
    Blue,
    Yellow,
    Green,
}

impl rand::distributions::Distribution<Color> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Color {
        match rng.gen_range(0..=3) {
            0 => Color::Red,
            1 => Color::Blue,
            2 => Color::Yellow,
            _ => Color::Green,
        }
    }
}

#[derive(Debug)]
pub struct App {
    phase: Phase,
    pattern: Vec<Color>,
    idx: usize,
    interval: Option<Interval>,
}

#[derive(Clone, Debug)]
pub enum Msg {
    NextColor,
    Clicked(Color),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let pattern = vec![rand::random()];
        let mut app = App {
            pattern,
            phase: Phase::default(),
            idx: 0,
            interval: None,
        };
        app.renew_interval(link);
        app
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let wedges: Vec<Html> = [Color::Green, Color::Red, Color::Yellow, Color::Blue]
            .iter()
            .cloned()
            .map(|color| {
                html! {
                    <Wedge
                        phase={self.phase}
                        {color}
                        onclick={ ctx.link().callback(move |_| Msg::Clicked(color)) }
                        highlighted={self.pattern[self.idx] == color}
                     />
                }
            })
            .collect();
        html! {
            <div class="container">
                <div class="wheel">
                    { wedges }
                    // <Wedge phase={self.phase} color={Color::Green} onclick={ ctx.link().callback(|_| Msg::Clicked(Color::Green)) } />
                    // <Wedge phase={self.phase} color={Color::Red} onclick={ ctx.link().callback(|_| Msg::Clicked(Color::Red)) } />
                    // <Wedge phase={self.phase} color={Color::Yellow} onclick={ ctx.link().callback(|_| Msg::Clicked(Color::Yellow)) } />
                    // <Wedge phase={self.phase} color={Color::Blue} onclick={ ctx.link().callback(|_| Msg::Clicked(Color::Blue)) } />
                    <div class="center">
                        <img src="https://www.simondata.com/wp-content/uploads/2021/01/footer-icon-logo.svg" alt="" />
                    </div>
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NextColor => self.advance_pattern(ctx),
            Msg::Clicked(color) => {
                log::warn!("{color:?} clicked!");
                if color == self.pattern[self.idx] {
                    self.advance_pattern(ctx);
                } else {
                    gloo_dialogs::alert("Wrong!");
                    self.restart_app(ctx);
                }
            }
        };
        log::warn!("{self:?}");
        true
    }
}

impl App {
    fn restart_app(&mut self, ctx: &Context<Self>) {
        self.phase = Phase::default();
        self.idx = 0;
        self.renew_interval(ctx.link().clone());
        self.pattern = vec![rand::random()];
    }

    fn is_pattern_done(&self) -> bool {
        self.idx >= self.pattern.len()
    }

    fn advance_pattern(&mut self, ctx: &Context<Self>) {
        self.idx += 1;
        if self.is_pattern_done() {
            if self.phase == Phase::Guess {
                self.pattern.push(rand::random());
            }
            self.idx = 0;
            self.cancel_interval();
            self.phase = match self.phase {
                Phase::Play => Phase::Guess,
                Phase::Guess => {
                    self.renew_interval(ctx.link().clone());
                    Phase::Play
                }
            };
        }
    }

    fn cancel_interval(&mut self) {
        if let Some(interval) = self.interval.take() {
            interval.cancel();
        }
    }

    fn renew_interval(&mut self, link: Scope<Self>) {
        self.cancel_interval();
        self.interval = Some(Interval::new(1000, move || {
            link.send_message(Msg::NextColor)
        }));
    }
}

#[derive(Properties, PartialEq)]
struct WedgeProps {
    phase: Phase,
    color: Color,
    onclick: Callback<MouseEvent>,
    highlighted: bool,
}

#[function_component(Wedge)]
fn wedge(props: &WedgeProps) -> Html {
    let pointer_events = match props.phase {
        Phase::Play => "none",
        Phase::Guess => "auto",
    };
    let style = format!("pointer-events:{pointer_events};");
    let class = classes!(
        "slice",
        &props.color.to_string(),
        (props.phase == Phase::Play && props.highlighted).then_some("hover")
    );
    let onclick = props.onclick.clone();
    html! {
        <div { class } { onclick } { style } ></div>
    }
}
