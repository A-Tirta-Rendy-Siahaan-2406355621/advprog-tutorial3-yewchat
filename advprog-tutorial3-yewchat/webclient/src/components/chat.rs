use serde::{Deserialize, Serialize};
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    current_user: String,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            current_user: username,
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let value = input.value();
                    let message_text = value.trim();
                    if message_text.is_empty() {
                        return false;
                    }

                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(message_text.to_string()),
                        data_array: None,
                    };

                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        let submit_on_enter = ctx.link().batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                Some(Msg::SubmitMessage)
            } else {
                None
            }
        });

        html! {
            <div class="flex w-screen h-screen bg-slate-50 text-slate-900">
                <div class="hidden md:flex flex-none w-72 h-screen flex-col border-r border-slate-200 bg-white">
                    <div class="border-b border-slate-200 p-5">
                        <div class="text-xs font-semibold uppercase tracking-wide text-blue-600">{"YewChat Lab"}</div>
                        <div class="mt-1 text-2xl font-bold text-slate-900">{"Creative Room"}</div>
                        <div class="mt-2 text-sm leading-5 text-slate-500">{"A realtime space for short ideas, quick feedback, and small wins."}</div>
                    </div>
                    <div class="flex items-center justify-between px-5 py-4">
                        <div class="text-sm font-semibold text-slate-700">{"Online users"}</div>
                        <div class="rounded-full bg-emerald-100 px-3 py-1 text-xs font-semibold text-emerald-700">
                            {format!("{} live", self.users.len())}
                        </div>
                    </div>
                    {
                        if self.users.is_empty() {
                            html! {
                                <div class="mx-5 rounded-lg border border-dashed border-slate-300 p-4 text-sm text-slate-500">
                                    {"Waiting for the first explorer to enter the room."}
                                </div>
                            }
                        } else {
                            self.users.clone().iter().map(|u| {
                                html! {
                                    <div class="mx-4 mb-3 flex items-center gap-3 rounded-lg border border-slate-100 bg-slate-50 p-3">
                                        <img class="h-11 w-11 rounded-full border border-white shadow-sm" src={u.avatar.clone()} alt="avatar"/>
                                        <div class="min-w-0 flex-grow">
                                            <div class="truncate text-sm font-semibold text-slate-800">{u.name.clone()}</div>
                                            <div class="text-xs text-slate-500">{"Ready to chat"}</div>
                                        </div>
                                        <div class="h-2.5 w-2.5 rounded-full bg-emerald-500"></div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    }
                    <div class="mt-auto border-t border-slate-200 p-5">
                        <div class="text-xs font-semibold uppercase tracking-wide text-slate-400">{"Room mood"}</div>
                        <div class="mt-3 grid grid-cols-3 gap-2 text-center text-xs font-semibold">
                            <div class="rounded-md bg-blue-50 px-2 py-3 text-blue-700">{"Focus"}</div>
                            <div class="rounded-md bg-amber-50 px-2 py-3 text-amber-700">{"Play"}</div>
                            <div class="rounded-md bg-emerald-50 px-2 py-3 text-emerald-700">{"Ship"}</div>
                        </div>
                    </div>
                </div>
                <div class="flex min-w-0 grow flex-col">
                    <div class="flex h-20 items-center justify-between border-b border-slate-200 bg-white px-5">
                        <div>
                            <div class="text-xl font-bold text-slate-900">{"YewChat Studio"}</div>
                            <div class="text-sm text-slate-500">{"Logged in as "}{self.current_user.clone()}</div>
                        </div>
                        <div class="hidden items-center gap-3 sm:flex">
                            <div class="rounded-full border border-slate-200 px-4 py-2 text-sm text-slate-600">
                                {format!("{} messages", self.messages.len())}
                            </div>
                            <div class="rounded-full bg-slate-900 px-4 py-2 text-sm font-semibold text-white">
                                {"Realtime"}
                            </div>
                        </div>
                    </div>
                    <div class="min-h-0 w-full grow overflow-auto border-b border-slate-200 bg-slate-50 px-4 py-6 sm:px-8">
                        {
                            if self.messages.is_empty() {
                                html! {
                                    <div class="mx-auto flex h-full max-w-2xl flex-col items-center justify-center text-center">
                                        <div class="flex -space-x-3">
                                            <img class="h-14 w-14 rounded-full border-4 border-white shadow-sm" src="https://api.dicebear.com/7.x/adventurer-neutral/svg?seed=orbit" alt="avatar orbit"/>
                                            <img class="h-14 w-14 rounded-full border-4 border-white shadow-sm" src="https://api.dicebear.com/7.x/adventurer-neutral/svg?seed=spark" alt="avatar spark"/>
                                            <img class="h-14 w-14 rounded-full border-4 border-white shadow-sm" src="https://api.dicebear.com/7.x/adventurer-neutral/svg?seed=wave" alt="avatar wave"/>
                                        </div>
                                        <div class="mt-6 text-2xl font-bold text-slate-900">{"Start with a spark"}</div>
                                        <p class="mt-3 max-w-md text-sm leading-6 text-slate-500">
                                            {"Send a tiny idea, a question, or a GIF link. The room becomes interesting after the first message."}
                                        </p>
                                        <div class="mt-6 grid w-full gap-3 sm:grid-cols-3">
                                            <div class="rounded-lg border border-slate-200 bg-white p-4 text-left text-sm text-slate-600 shadow-sm">{"What is one thing you learned today?"}</div>
                                            <div class="rounded-lg border border-slate-200 bg-white p-4 text-left text-sm text-slate-600 shadow-sm">{"Drop a small win from your coding session."}</div>
                                            <div class="rounded-lg border border-slate-200 bg-white p-4 text-left text-sm text-slate-600 shadow-sm">{"Share a GIF link when words are too slow."}</div>
                                        </div>
                                    </div>
                                }
                            } else {
                                self.messages.iter().map(|m| {
                                    let default_profile = UserProfile {
                                        name: m.from.clone(),
                                        avatar: format!(
                                            "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                            m.from
                                        ),
                                    };
                                    let user = self.users.iter().find(|u| u.name == m.from).unwrap_or(&default_profile);
                                    let is_me = m.from == self.current_user;
                                    let row_class = if is_me {
                                        "mb-5 flex items-end justify-end gap-3"
                                    } else {
                                        "mb-5 flex items-end justify-start gap-3"
                                    };
                                    let bubble_class = if is_me {
                                        "max-w-xl rounded-lg rounded-br-sm bg-blue-600 px-4 py-3 text-white shadow-sm"
                                    } else {
                                        "max-w-xl rounded-lg rounded-bl-sm border border-slate-200 bg-white px-4 py-3 text-slate-700 shadow-sm"
                                    };
                                    let meta_class = if is_me {
                                        "mb-1 text-xs font-semibold text-blue-100"
                                    } else {
                                        "mb-1 text-xs font-semibold text-slate-500"
                                    };
                                    let text_class = if is_me {
                                        "text-sm leading-6 text-white"
                                    } else {
                                        "text-sm leading-6 text-slate-700"
                                    };

                                    html! {
                                        <div class={row_class}>
                                            { if !is_me {
                                                html! { <img class="h-9 w-9 rounded-full border border-white shadow-sm" src={user.avatar.clone()} alt="avatar"/> }
                                            } else {
                                                html! {}
                                            }}
                                            <div class={bubble_class}>
                                                <div class={meta_class}>{m.from.clone()}</div>
                                                {
                                                    if m.message.ends_with(".gif") {
                                                        html! { <img class="mt-2 max-h-72 rounded-md" src={m.message.clone()} alt="gif message"/> }
                                                    } else {
                                                        html! { <p class={text_class}>{m.message.clone()}</p> }
                                                    }
                                                }
                                            </div>
                                            { if is_me {
                                                html! { <img class="h-9 w-9 rounded-full border border-white shadow-sm" src={user.avatar.clone()} alt="avatar"/> }
                                            } else {
                                                html! {}
                                            }}
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                        }
                    </div>
                    <div class="flex min-h-20 items-center gap-3 bg-white px-4 py-4 sm:px-6">
                        <input ref={self.chat_input.clone()} onkeypress={submit_on_enter} type="text" placeholder="Write something worth replying to..." class="block h-12 min-w-0 flex-1 rounded-full border border-slate-200 bg-slate-50 px-5 text-sm outline-none transition focus:border-blue-400 focus:bg-white focus:text-slate-900" name="message" required=true />
                        <button onclick={submit} class="flex h-12 w-12 items-center justify-center rounded-full bg-blue-600 text-white shadow-sm transition hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-300" aria-label="Send message">
                            <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 fill-current">
                                <path d="M0 0h24v24H0z" fill="none"></path><path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                            </svg>
                        </button>
                    </div>
                </div>
                <div class="hidden w-80 flex-none flex-col border-l border-slate-200 bg-white xl:flex">
                    <div class="border-b border-slate-200 p-5">
                        <div class="text-sm font-semibold text-slate-900">{"Creative prompts"}</div>
                        <div class="mt-1 text-sm leading-5 text-slate-500">{"Use these when the chat needs a better opening line."}</div>
                    </div>
                    <div class="space-y-3 p-5">
                        {
                            [
                                "Ask for a code review in one sentence.",
                                "Share the bug you finally understood.",
                                "Post the smallest idea you can test today.",
                                "Turn a boring status update into a useful question.",
                            ].iter().map(|prompt| {
                                html! {
                                    <div class="rounded-lg border border-slate-200 bg-slate-50 p-4 text-sm leading-6 text-slate-600">
                                        {prompt}
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                    <div class="mt-auto p-5">
                        <div class="rounded-lg bg-slate-900 p-5 text-white">
                            <div class="text-sm font-semibold">{"Tiny rule"}</div>
                            <div class="mt-2 text-sm leading-6 text-slate-300">{"Good chat rooms are made from short messages that invite one more reply."}</div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
