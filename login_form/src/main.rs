//yewdux is a simple state management for yew applications.
//using gloo for making request and for console.log()
use yewdux::{prelude::*, storage};
use yew::prelude::*;
use gloo::{console::log, net::http::Request};
use web_sys::{HtmlInputElement};
use serde::{Serialize, Deserialize};

#[derive( Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserData{
    pub email:String,
    pub password:String,
}

impl Store for UserData {
    fn new() -> Self {
        storage::load(storage::Area::Local)
            .expect("Unable to load state")
            .unwrap_or_default()
    }

    fn should_notify(&self, _: &Self) -> bool{
        storage::save(self, storage::Area::Local).expect("Unable to save state");
        true
    }
}






#[function_component(Form)]
pub fn form()-> Html{
    
    let (state, dispatch) = use_store::<UserData>();

        
       
        let email =  dispatch.reduce_mut_callback_with(|state, event:Event| {
            let email = event.target_unchecked_into::<HtmlInputElement>().value();
            state.email = email;
            
        });

        

        let password =  dispatch.reduce_mut_callback_with(|state, event:Event| {
            let password = event.target_unchecked_into::<HtmlInputElement>().value();
            state.password = password;
        });
        let submit = dispatch.reduce_mut_callback_with(|state, _| {
            
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
            
                let request= Request::post("http://127.0.0.1:3000/login")
                .json(&state)
                .unwrap()
                .send()
                .await
                .unwrap();
            let status_code=request.status();
            if status_code>=200 && status_code <=400 
            {
                if request.headers().has("token") {
                    log!("User is admin")
                }
                else{
                    log!("user is an employee")
                }
                
                
            
              
            }
            else{
                log!(request.text().await.unwrap());
            }
            });

        });

        html!(
            <>
        <div>
          <label for="email" >{ "Email" }</label>
          <input type="email" id="email" placeholder={state.email.clone()} onchange={email}/>
          <br />
          <label for="password">{ "Password" }</label>
          <input type="password" id="password" onchange={password} />
          <br />
          <button onclick={submit}>{ "submit" }</button>
        </div>
        <div>
           <h4>{ "Details" }</h4>
           <p>{"Email: "} {&state.email} </p>
        </div>
        </>
        ) 
    
}
fn main() {
    yew::start_app::<Form>();
}
