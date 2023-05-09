use crate::BoltContext;
use crate::Msg;
use crate::Page;
use yew::{html, Html};

pub fn sidebar(bctx: &mut BoltContext, active: u8) -> Html {
 let link = bctx.link.as_ref().unwrap();
    
    html! {
        <div class="sidebar1">
            <div class={if active == 0 {"sidebaritem sidebaritem-selected pointer"} else {"sidebaritem pointer"} } onclick={link.callback(|_| Msg::SwitchPage(Page::Home))}>
                <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 1024 1024" height="25px" width="25px" xmlns="http://www.w3.org/2000/svg"><path d="M917.7 148.8l-42.4-42.4c-1.6-1.6-3.6-2.3-5.7-2.3s-4.1.8-5.7 2.3l-76.1 76.1a199.27 199.27 0 0 0-112.1-34.3c-51.2 0-102.4 19.5-141.5 58.6L432.3 308.7a8.03 8.03 0 0 0 0 11.3L704 591.7c1.6 1.6 3.6 2.3 5.7 2.3 2 0 4.1-.8 5.7-2.3l101.9-101.9c68.9-69 77-175.7 24.3-253.5l76.1-76.1c3.1-3.2 3.1-8.3 0-11.4zM578.9 546.7a8.03 8.03 0 0 0-11.3 0L501 613.3 410.7 523l66.7-66.7c3.1-3.1 3.1-8.2 0-11.3L441 408.6a8.03 8.03 0 0 0-11.3 0L363 475.3l-43-43a7.85 7.85 0 0 0-5.7-2.3c-2 0-4.1.8-5.7 2.3L206.8 534.2c-68.9 68.9-77 175.7-24.3 253.5l-76.1 76.1a8.03 8.03 0 0 0 0 11.3l42.4 42.4c1.6 1.6 3.6 2.3 5.7 2.3s4.1-.8 5.7-2.3l76.1-76.1c33.7 22.9 72.9 34.3 112.1 34.3 51.2 0 102.4-19.5 141.5-58.6l101.9-101.9c3.1-3.1 3.1-8.2 0-11.3l-43-43 66.7-66.7c3.1-3.1 3.1-8.2 0-11.3l-36.6-36.2z"></path></svg>
                {"Requests"}
            </div>

            <div class={if active == 1 {"sidebaritem sidebaritem-selected pointer"} else {"sidebaritem pointer"} } onclick={link.callback(|_| Msg::SwitchPage(Page::Collections) )}>
                <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 16 16" height="25px" width="25px" xmlns="http://www.w3.org/2000/svg"><path d="M0 13a1.5 1.5 0 0 0 1.5 1.5h13A1.5 1.5 0 0 0 16 13V6a1.5 1.5 0 0 0-1.5-1.5h-13A1.5 1.5 0 0 0 0 6v7zM2 3a.5.5 0 0 0 .5.5h11a.5.5 0 0 0 0-1h-11A.5.5 0 0 0 2 3zm2-2a.5.5 0 0 0 .5.5h7a.5.5 0 0 0 0-1h-7A.5.5 0 0 0 4 1z"></path></svg>
                {"Collections"}
            </div>

            // <div class="sidebaritem pointer">
                // <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 24 24" height="25px" width="25px" xmlns="http://www.w3.org/2000/svg"><path fill="none" stroke-width="2" d="M8.9997,0.99999995 L8.9997,8.0003 L1.9997,20.0003 L1.9997,23.0003 L21.9997,23.0003 L21.9997,20.0003 L14.9997,8.0003 L14.9997,0.99999995 M15,18 C15.5522847,18 16,17.5522847 16,17 C16,16.4477153 15.5522847,16 15,16 C14.4477153,16 14,16.4477153 14,17 C14,17.5522847 14.4477153,18 15,18 Z M9,20 C9.55228475,20 10,19.5522847 10,19 C10,18.4477153 9.55228475,18 9,18 C8.44771525,18 8,18.4477153 8,19 C8,19.5522847 8.44771525,20 9,20 Z M18,13 C11,9.99999996 12,17.0000002 6,14 M5.9997,1.0003 L17.9997,1.0003"></path></svg>
                // {"Test"}
            // </div>
        </div>
    }
}
