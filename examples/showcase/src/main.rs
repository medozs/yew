#![recursion_limit="128"]

#[macro_use]
extern crate yew;
extern crate counter;
extern crate crm;
extern crate custom_components;
extern crate dashboard;
extern crate fragments;
extern crate game_of_life;
extern crate inner_html;
extern crate large_table;
extern crate mount_point;
extern crate npm_and_rest;
extern crate textarea;
extern crate timer;
extern crate todomvc;
extern crate two_apps;

use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::dialog::DialogService;
use yew::services::storage::{StorageService, Area};
use yew::services::fetch::FetchService;
use yew::services::websocket::WebSocketService;
use yew::services::interval::IntervalService;
use yew::services::timeout::TimeoutService;
use counter::Model as Counter;
use crm::Model as Crm;
use custom_components::Model as CustomComponents;
use dashboard::Model as Dashboard;
use fragments::Model as Fragments;
use game_of_life::GameOfLife;
use inner_html::Model as InnerHtml;
use large_table::Model as LargeTable;
use mount_point::Model as MountPoint;
use npm_and_rest::Model as NpmAndRest;
use npm_and_rest::gravatar::GravatarService;
use npm_and_rest::ccxt::CcxtService;
use textarea::Model as Textarea;
use timer::Model as Timer;
use todomvc::Model as Todomvc;
use two_apps::Model as TwoApps;

struct Context {
    console: ConsoleService,
    storage: StorageService,
    dialog: DialogService,
    web: FetchService,
    ws: WebSocketService,
    interval: IntervalService,
    gravatar: GravatarService,
    ccxt: CcxtService,
    timeout: TimeoutService,
    two_apps: two_apps::Context,
}

impl AsMut<ConsoleService> for Context {
    fn as_mut(&mut self) -> &mut ConsoleService {
        &mut self.console
    }
}

impl AsMut<StorageService> for Context {
    fn as_mut(&mut self) -> &mut StorageService {
        &mut self.storage
    }
}

impl AsMut<DialogService> for Context {
    fn as_mut(&mut self) -> &mut DialogService {
        &mut self.dialog
    }
}

impl AsMut<FetchService> for Context {
    fn as_mut(&mut self) -> &mut FetchService {
        &mut self.web
    }
}

impl AsMut<WebSocketService> for Context {
    fn as_mut(&mut self) -> &mut WebSocketService {
        &mut self.ws
    }
}

impl AsMut<IntervalService> for Context {
    fn as_mut(&mut self) -> &mut IntervalService {
        &mut self.interval
    }
}

impl custom_components::Printer for Context {
    fn print(&mut self, data: &str) {
        self.console.log(data);
    }
}

impl AsMut<GravatarService> for Context {
    fn as_mut(&mut self) -> &mut GravatarService {
        &mut self.gravatar
    }
}

impl AsMut<CcxtService> for Context {
    fn as_mut(&mut self) -> &mut CcxtService {
        &mut self.ccxt
    }
}

impl AsMut<TimeoutService> for Context {
    fn as_mut(&mut self) -> &mut TimeoutService {
        &mut self.timeout
    }
}

impl AsMut<two_apps::Context> for Context {
    fn as_mut(&mut self) -> &mut two_apps::Context {
        &mut self.two_apps
    }
}

enum Scene {
    NotSelected,
    Counter,
    Crm,
    CustomComponents,
    Dashboard,
    Fragments,
    GameOfLife,
    InnerHtml,
    LargeTable,
    MountPoint,
    NpmAndRest,
    Textarea,
    Timer,
    Todomvc,
    TwoApps,
}

enum Msg {
    SwitchTo(Scene),
}

impl Component<Context> for Scene {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Scene::NotSelected
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::SwitchTo(scene) => {
                *self = scene;
                true
            }
        }
    }
}

impl Renderable<Context, Scene> for Scene {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <p>{ "Showcase" }</p>
            <button onclick=|_| Msg::SwitchTo(Scene::NotSelected),>{ "Home" }</button>
            <button onclick=|_| Msg::SwitchTo(Scene::Counter),>{ "Counter" }</button>
            <button onclick=|_| Msg::SwitchTo(Scene::Crm),>{ "Crm" }</button>
            <button onclick=|_| Msg::SwitchTo(Scene::CustomComponents),>{ "CustomComponents" }</button>
            <button onclick=|_| Msg::SwitchTo(Scene::Dashboard),>{ "Dashboard" }</button>
            <button onclick=|_| Msg::SwitchTo(Scene::Fragments),>{ "Fragments" }</button>
            <button onclick=|_| Msg::SwitchTo(Scene::GameOfLife),>{ "GameOfLife" }</button>
            <button onclick=|_| Msg::SwitchTo(Scene::InnerHtml),>{ "InnerHtml" }</button>
            <button onclick=|_| Msg::SwitchTo(Scene::LargeTable),>{ "LargeTable" }</button>
            <button onclick=|_| Msg::SwitchTo(Scene::MountPoint),>{ "MountPoint" }</button>
            <button onclick=|_| Msg::SwitchTo(Scene::NpmAndRest),>{ "NpmAndRest" }</button>
            <button onclick=|_| Msg::SwitchTo(Scene::Textarea),>{ "Textarea" }</button>
            <button onclick=|_| Msg::SwitchTo(Scene::Timer),>{ "Timer" }</button>
            <button onclick=|_| Msg::SwitchTo(Scene::Todomvc),>{ "Todomvc" }</button>
            <button onclick=|_| Msg::SwitchTo(Scene::TwoApps),>{ "TwoApps" }</button>
            { self.view_scene() }
        }
    }
}

impl Scene {
    fn view_scene(&self) -> Html<Context, Self> {
        match *self {
            Scene::NotSelected => {
                html! {
                    <p>{ "Select the scene, please." }</p>
                }
            }
            Scene::Counter => {
                html! {
                    <Counter: />
                }
            }
            Scene::Crm => {
                html! {
                    <Crm: />
                }
            }
            Scene::CustomComponents => {
                html! {
                    <CustomComponents: />
                }
            }
            Scene::Dashboard => {
                html! {
                    <Dashboard: />
                }
            }
            Scene::Fragments => {
                html! {
                    <Fragments: />
                }
            }
            Scene::GameOfLife => {
                html! {
                    <GameOfLife: />
                }
            }
            Scene::InnerHtml => {
                html! {
                    <InnerHtml: />
                }
            }
            Scene::LargeTable => {
                html! {
                    <LargeTable: />
                }
            }
            Scene::MountPoint => {
                html! {
                    <MountPoint: />
                }
            }
            Scene::NpmAndRest => {
                html! {
                    <NpmAndRest: />
                }
            }
            Scene::Textarea => {
                html! {
                    <Textarea: />
                }
            }
            Scene::Timer => {
                html! {
                    <Timer: />
                }
            }
            Scene::Todomvc => {
                html! {
                    <Todomvc: />
                }
            }
            Scene::TwoApps => {
                html! {
                    <TwoApps: />
                }
            }
        }
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        console: ConsoleService::new(),
        storage: StorageService::new(Area::Local),
        dialog: DialogService::new(),
        web: FetchService::new(),
        ws: WebSocketService::new(),
        interval: IntervalService::new(),
        gravatar: GravatarService::new(),
        ccxt: CcxtService::new(),
        timeout: TimeoutService::new(),
        two_apps: two_apps::Context::new(),
    };
    let app: App<_, Scene> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}

