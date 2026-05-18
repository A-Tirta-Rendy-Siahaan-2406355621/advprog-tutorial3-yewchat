use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");
    let can_enter = username.trim().len() > 0;

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = username.trim().to_string())
    };

    html! {
       <div class="flex min-h-screen w-screen bg-slate-950 text-white">
            <div class="grid w-full lg:grid-cols-[minmax(0,1fr)_520px]">
                <div class="flex flex-col justify-between px-6 py-8 sm:px-10 lg:px-16">
                    <div>
                        <div class="inline-flex items-center gap-3 rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-200">
                            <span class="h-2.5 w-2.5 rounded-full bg-emerald-400"></span>
                            {"WebSocket room is ready"}
                        </div>
                        <div class="mt-16 max-w-2xl">
                            <h1 class="text-5xl font-black leading-tight tracking-normal sm:text-6xl">{"YewChat Lab"}</h1>
                            <p class="mt-6 text-lg leading-8 text-slate-300">
                                {"A small realtime chat studio for practicing async communication, sharing quick ideas, and making the browser feel less lonely."}
                            </p>
                        </div>
                    </div>
                    <div class="mt-12 grid gap-4 sm:grid-cols-3">
                        <div class="rounded-lg border border-white/10 bg-white/5 p-4">
                            <div class="text-sm font-semibold text-white">{"Fast"}</div>
                            <div class="mt-2 text-sm leading-6 text-slate-400">{"Messages move through an open websocket connection."}</div>
                        </div>
                        <div class="rounded-lg border border-white/10 bg-white/5 p-4">
                            <div class="text-sm font-semibold text-white">{"Visual"}</div>
                            <div class="mt-2 text-sm leading-6 text-slate-400">{"Every user gets a generated avatar and clear presence state."}</div>
                        </div>
                        <div class="rounded-lg border border-white/10 bg-white/5 p-4">
                            <div class="text-sm font-semibold text-white">{"Creative"}</div>
                            <div class="mt-2 text-sm leading-6 text-slate-400">{"Prompts help turn empty chat into useful conversation."}</div>
                        </div>
                    </div>
                </div>
                <div class="flex items-center justify-center border-l border-white/10 bg-white px-6 py-10 text-slate-900">
                    <div class="w-full max-w-md">
                        <div class="mb-8 flex -space-x-4">
                            <img class="h-16 w-16 rounded-full border-4 border-white shadow-md" src="https://api.dicebear.com/7.x/adventurer-neutral/svg?seed=yew" alt="avatar yew"/>
                            <img class="h-16 w-16 rounded-full border-4 border-white shadow-md" src="https://api.dicebear.com/7.x/adventurer-neutral/svg?seed=rust" alt="avatar rust"/>
                            <img class="h-16 w-16 rounded-full border-4 border-white shadow-md" src="https://api.dicebear.com/7.x/adventurer-neutral/svg?seed=wasm" alt="avatar wasm"/>
                        </div>
                        <div class="text-sm font-semibold uppercase tracking-wide text-blue-600">{"Enter the room"}</div>
                        <h2 class="mt-2 text-3xl font-black tracking-normal text-slate-900">{"Pick a username"}</h2>
                        <p class="mt-3 text-sm leading-6 text-slate-500">
                            {"Choose a name that other connected clients will see in the live user list and message bubbles."}
                        </p>
                        <div class="mt-8">
                            <label class="mb-2 block text-sm font-semibold text-slate-700" for="username">{"Username"}</label>
                            <input id="username" {oninput} class="h-12 w-full rounded-lg border border-slate-300 bg-slate-50 px-4 text-slate-900 outline-none transition focus:border-blue-500 focus:bg-white focus:ring-2 focus:ring-blue-100" placeholder="for example: indra" />
                        </div>
                        <div class="mt-5">
                            {
                                if can_enter {
                                    html! {
                                        <Link<Route> to={Route::Chat}>
                                            <button type="button" {onclick} class="flex h-12 w-full items-center justify-center rounded-lg bg-blue-600 px-5 text-sm font-bold uppercase tracking-wide text-white shadow-sm transition hover:bg-blue-700">
                                                {"Start chatting"}
                                            </button>
                                        </Link<Route>>
                                    }
                                } else {
                                    html! {
                                        <button type="button" disabled=true class="flex h-12 w-full cursor-not-allowed items-center justify-center rounded-lg bg-slate-200 px-5 text-sm font-bold uppercase tracking-wide text-slate-400">
                                            {"Start chatting"}
                                        </button>
                                    }
                                }
                            }
                        </div>
                        <div class="mt-8 rounded-lg border border-slate-200 bg-slate-50 p-4">
                            <div class="text-sm font-semibold text-slate-900">{"Starter sentence"}</div>
                            <div class="mt-2 text-sm leading-6 text-slate-500">{"Today I learned something small but useful: ..."}</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
