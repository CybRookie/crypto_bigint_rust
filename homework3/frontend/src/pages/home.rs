// use reqwasm::http::{Request, Response};
use yew::prelude::*;
use logic::logic::config::{ConfigVariant, ConfigSymmetric, ConfigDF, ConfigRSA, Cipher, Mode};
use yew::services::ConsoleService;
use web_sys::{window, Document, Element, HtmlCollection, HtmlElement, HtmlFormElement};
use futures::executor::block_on;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};
use serde::Deserialize;

// const DEFAULT_SELECT_OPTIONS_CLASS_NAME: &str = "default-select-options";

#[derive(Clone)]
pub struct MainPage {
    link: ComponentLink<Self>,
    selected_cipher: Option<Cipher>,
    selected_cipher_incorrect: bool,
    selected_mode: Option<Mode>,
    selected_mode_incorrect: bool,
    string_target: Option<String>,
    symmetric_key: Option<String>,
    rsa_exponent: Option<String>,
    rsa_modulus: Option<String>,
    rsa_threads: Option<String>,
    df_shared_prime: Option<String>,
    df_shared_base: Option<String>,
    df_secret_a: Option<String>,
    df_secret_b: Option<String>,
}

// Variants of messages that will signal a change of form's state.
#[derive(Clone)]
pub enum MainPageMsg {
    CipherSelect(String),
    CipherSelectIncorrect,
    EncryptionModeSelect(String),
    EncryptionModeSelectIncorrect,
    TargetString(String),
    SymmetricKey(String),
    DFPrime(String),
    DFBase(String),
    DFSecretA(String),
    DFSecretB(String),
    DFGenerate,
    SymmetricEncrypt,
    SymmetricDecrypt,
    RSAEncrypt,
    RSADecrypt,
    RSAGenerate,
    RSABruteforce,
    RSAExponent(String),
    RSAModulus(String),
    RSAThreads(String),
    ResetFormFields,
}

impl Component for MainPage {
    type Message = MainPageMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            selected_cipher: None,
            selected_cipher_incorrect: false,
            selected_mode: None,
            selected_mode_incorrect: false,
            string_target: None,
            symmetric_key: None,
            rsa_exponent: None,
            rsa_modulus: None,
            rsa_threads: None,
            df_shared_prime: None,
            df_shared_base: None,
            df_secret_a: None,
            df_secret_b: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let main_page_component_clone = (*self).clone();
        // Handle the incoming messages, process them, and determine if the state should be updated
        // and the appropriate section reloaded.
        match msg {
            MainPageMsg::CipherSelect(content) => {
                // Determine cipher type to use, Caesar, Vigenere, RSA or Diffie-Hellman key exchange algorithm.
                let cipher = match content.as_str() {
                    "caesar" => Cipher::Caesar,
                    "vigenere" => Cipher::Vigenere,
                    "df" => Cipher::DiffieHellman,
                    "rsa" => Cipher::RSA,
                    _ => {
                        // If a frontend form was modified and an incorrect value was received, reload the page.
                        self.reset_main_page_component_values();
                        return true;
                    },
                };

                // let html_element_collection = window().expect("A page's window could not be acquired.")
                //     .document().expect("A page's document could not be acquired.")
                //     .get_elements_by_class_name(DEFAULT_SELECT_OPTIONS_CLASS_NAME);
                //
                // // window().expect("A page's window could not be acquired.").location().reload();
                //
                // let mut index: u32 = 0;
                // loop {
                //     let target_element = html_element_collection.item(index);
                //
                //     match target_element {
                //         Some(element) => {
                //             // let _result = element.set_attribute("selected", "");
                //             let _result = element.set_attribute("selected", "selected");
                //             // element.selected = false;
                //             // let html_element: HtmlElement = unsafe { mem::transmute(element) };
                //             // html_element.focus();
                //         },
                //         None => {
                //             break;
                //         },
                //     }
                //
                //     index += 1;
                // }

                self.reset_main_page_component_values();
                self.selected_cipher = Option::from(cipher);
                true
            },
            MainPageMsg::CipherSelectIncorrect => {
                self.reset_main_page_component_values();
                self.selected_cipher_incorrect = true;
                return true;
            },
            MainPageMsg::EncryptionModeSelect(content) => {
                // Determine cipher type to use, Caesar, Vigenere, RSA or Diffie-Hellman key exchange algorithm.
                let mode = match content.as_str() {
                    "symmetric-encrypt" if ((self.selected_cipher == Some(Cipher::Caesar)) || (self.selected_cipher == Some(Cipher::Vigenere))) => Mode::Encode,
                    "symmetric-decrypt" if ((self.selected_cipher == Some(Cipher::Caesar)) || (self.selected_cipher == Some(Cipher::Vigenere))) => Mode::Decode,
                    "rsa-encrypt" if self.selected_cipher == Some(Cipher::RSA) => Mode::Encode,
                    "rsa-decrypt" if self.selected_cipher == Some(Cipher::RSA) => Mode::Decode,
                    "rsa-generate" if self.selected_cipher == Some(Cipher::RSA) => Mode::Generate,
                    "rsa-bruteforce" if self.selected_cipher == Some(Cipher::RSA) => Mode::Bruteforce,
                    _ => {
                        // If a frontend form was modified and an incorrect value was received, reload the page.
                        let selected_cipher_mode_buffer = Option::from(main_page_component_clone.selected_cipher.unwrap());
                        self.reset_main_page_component_values();
                        self.selected_cipher = selected_cipher_mode_buffer;
                        return true;
                    },
                };

                let selected_cipher_mode_buffer = Option::from(main_page_component_clone.selected_cipher.unwrap());
                self.reset_main_page_component_values();
                self.selected_cipher = selected_cipher_mode_buffer;
                self.selected_mode = Option::from(mode);
                true
            },
            MainPageMsg::EncryptionModeSelectIncorrect => {
                self.reset_main_page_component_values();
                self.selected_mode_incorrect = true;
                return true;
            },
            MainPageMsg::TargetString(content) => {
                self.string_target = Option::from(content);
                false
            },
            MainPageMsg::SymmetricKey(content) => {
                self.symmetric_key = Option::from(content);
                false
            },
            MainPageMsg::DFPrime(content) => {
                self.df_shared_prime =  match content.as_str() {
                    "none" => { None },
                    "" => { None },
                    _ => { Option::from(content) },
                };
                false
            },
            MainPageMsg::DFBase(content) => {
                self.df_shared_base =  match content.as_str() {
                    "none" => { None },
                    "" => { None },
                    _ => { Option::from(content) },
                };
                false
            },
            MainPageMsg::DFSecretA(content) => {
                self.df_secret_a =  match content.as_str() {
                    "none" => { None },
                    "" => { None },
                    _ => { Option::from(content) },
                };
                false
            },
            MainPageMsg::DFSecretB(content) => {
                self.df_secret_b =  match content.as_str() {
                    "none" => { None },
                    "" => { None },
                    _ => { Option::from(content) },
                };
                false
            },
            MainPageMsg::DFGenerate => {

                // let resp = Request::get("http://127.0.0.1:8080/hey")
                //     .send();
                // let resp = block_on(resp).unwrap();
                    // ConsoleService::info(format!("Response: {:?}", resp).as_ref());

                let request = Request::get("http://127.0.0.1:8080/hey")
                    .body(Nothing)
                    .expect("Could not build request.");

                let callback = self.link.callback(|response: Response<Json<Result<ISS, anyhow::Error>>>| {
                    let Json(data) = response.into_body();
                    Msg::ReceiveResponse(data)
                });


                    ConsoleService::info(format!("HALLLO!!!").as_ref());
                    window().expect("A page's window could not be acquired.").alert_with_message("HALLO!");
                // };
                // wasm_logger::init(wasm_logger::Config::default());
                // log::info!("Some info");
                // log::error!("Error message");
                // web_sys::console::log_1(&"Hello, world!".into());
                // ConsoleService::info(format!("HALLLO!!!").as_ref());
                // window().expect("A page's window could not be acquired.").alert_with_message("HALLO!");
                false
            },
            MainPageMsg::SymmetricEncrypt => { true },
            MainPageMsg::SymmetricDecrypt => { true },
            MainPageMsg::RSAEncrypt => { true },
            MainPageMsg::RSADecrypt => { true },
            MainPageMsg::RSAGenerate => { true },
            MainPageMsg::RSABruteforce => { true },
            MainPageMsg::RSAExponent(content) => {
                self.rsa_exponent = Option::from(content);
                false
            },
            MainPageMsg::RSAModulus(content) => {
                self.rsa_modulus = Option::from(content);
                false
            },
            MainPageMsg::RSAThreads(content) => {
                self.rsa_threads = Option::from(content);
                false
            },
            MainPageMsg::ResetFormFields => {
                // let element = window().expect("A page's window could not be acquired.")
                //     .document().expect("A page's document could not be acquired.")
                //     .get_element_by_id("algorithm").expect("A page's document could not be acquired.");
                // // element.set_attribute("selected", "selected");
                // let element_html_form: HtmlFormElement = Element::from(element);
                // element_html_form.reset();
                self.reset_main_page_component_values();
                true
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onchange_algorithm = self.link.callback(|event: ChangeData| {
            if let ChangeData::Select(select_element) = event {
                MainPageMsg::CipherSelect(select_element.value())
            } else {
                MainPageMsg::CipherSelectIncorrect
            }
        });
        html! {
            <>
                { MainPage::main_page_header() }
                { self.display_error_message() }
                <section>
                    <form action="">
                        <label for="algorithm">{ "Choose an algorithm to use:" }</label>
                        <select name="" id="algorithm" onchange={onchange_algorithm}>
                            <option id="algorithm-default-option" value="" selected=true disabled=true hidden=true>{ "Choose an algorithm" }</option>
                            <option value="caesar">{ "Caesar cipher" }</option>
                            <option value="vigenere">{ "Vigenere cipher" }</option>
                            <option value="rsa">{ "RSA cipher" }</option>
                            <option value="df">{ "Diffie-Hellman key-exchange" }</option>
                        </select>
                        { self.display_cipher_adjusted_form_part() }
                    </form>
                </section>
            </>
        }
    }
}

impl MainPage {
    fn reset_main_page_component_values(&mut self) {
        self.selected_cipher = None;
        self.selected_cipher_incorrect = false;
        self.selected_mode = None;
        self.selected_mode_incorrect = false;
        self.string_target = None;
        self.symmetric_key = None;
        self.rsa_exponent = None;
        self.rsa_modulus = None;
        self.rsa_threads = None;
        self.df_shared_prime = None;
        self.df_shared_base = None;
        self.df_secret_a = None;
        self.df_secret_b = None;
    }

    fn display_error_message(&self) -> Html {
        if self.selected_cipher_incorrect == true {
            html! {
                <section>
                    <p>
                        { "An incorrect algorithm was selected. Either the frontend code was modified or the error in the algorithm has happened. " }
                    </p>
                </section>
            }
        } else if self.selected_mode_incorrect == true {
            html! {
                <section>
                    <p>
                        { "An incorrect cipher mode was selected. Either the frontend code was modified or the error in the algorithm has happened. " }
                    </p>
                </section>
            }
        } else {
            html! {}
        }
    }

    fn main_page_header() -> Html {
        html! {
            <header>
                <h1>
                { "Welcome to the encryption webapp!" }
                </h1>
                <p>
                { "The following form allows encrypting/decrypting a target string with Caesar, Vigenere or RSA algorithms.
                Moreover, you can generate a random RSA keypair or run a Diffie-Hellman key-exchange algorithm on randomly generated values.
                The form below will dynamically adjust to the chosen options." }
                </p>
            </header>
        }
    }

    fn display_cipher_adjusted_form_part(&self) -> Html {
        let main_page_component = self.clone();
        match main_page_component.selected_cipher  {
            Some(cipher) if (cipher.eq(&Cipher::Caesar) || cipher.eq(&Cipher::Vigenere)) => {
                let onchange_symmetric_mode = self.link.callback(|event: ChangeData| {
                    if let ChangeData::Select(select_element) = event {
                        MainPageMsg::EncryptionModeSelect(select_element.value())
                    } else {
                        MainPageMsg::EncryptionModeSelectIncorrect
                    }
                });
                let oninput_symmetric_target = self.link.callback(|event: InputData| MainPageMsg::TargetString(event.value));
                let oninput_symmetric_key = self.link.callback(|event: InputData| MainPageMsg::SymmetricKey(event.value));
                html! {
                    <>
                        <label for="symmetric-modes">{ "Choose a mode for a symmetric cipher:" }</label>
                        <select name="" id="symmetric-modes" onchange={onchange_symmetric_mode}>
                            <option value="" selected=true disabled=true hidden=true>{ "Choose a symmetric algorithm mode" }</option>
                            <option value="symmetric-encrypt">{ "Encrypt" }</option>
                            <option value="symmetric-decrypt">{ "Decrypt" }</option>
                        </select>
                        <label for="symmetric-target">{ "Provide a target string for encryption/decryption:" }</label>
                        <input type="text" id="symmetric-target" oninput={oninput_symmetric_target}/>
                        <label for="symmetric-key">{ "Provide a key for encryption/decryption:" }</label>
                        <input type="text" id="symmetric-key" oninput={oninput_symmetric_key}/>
                        { self.display_adjusted_form_button() }
                        { self.display_reset_form_button() }
                    </>
                }
            },
            Some(cipher) if cipher.eq(&Cipher::DiffieHellman) => {
                let oninput_df_prime = self.link.callback(|event: InputData| MainPageMsg::DFPrime(event.value));
                let oninput_df_base = self.link.callback(|event: InputData| MainPageMsg::DFBase(event.value));
                let oninput_df_secret_a = self.link.callback(|event: InputData| MainPageMsg::DFSecretA(event.value));
                let oninput_df_secret_b = self.link.callback(|event: InputData| MainPageMsg::DFSecretB(event.value));
                let onclick_df_generate = self.link.callback(|_| MainPageMsg::DFGenerate);
                html! {
                    <>
                        <label for="df-prime">{ "Provide a Diffie-Hellman shared prime:" }</label>
                        <input type="text" id="df-prime" oninput={oninput_df_prime}/>
                        <label for="df-base">{ "Provide a Diffie-Hellman shared base:" }</label>
                        <input type="text" id="df-base" oninput={oninput_df_base}/>
                        <label for="df-secret-a">{ "Provide a Diffie-Hellman secret A:" }</label>
                        <input type="text" id="df-secret-a" oninput={oninput_df_secret_a}/>
                        <label for="df-secret-b">{ "Provide a Diffie-Hellman secret B:" }</label>
                        <input type="text" id="df-secret-b" oninput={oninput_df_secret_b}/>
                        <button type="button" onclick={onclick_df_generate}>{ "Generate" }</button>
                        { self.display_reset_form_button() }
                    </>
                }
            },
            Some(cipher) if cipher.eq(&Cipher::RSA) => {
                let onchange_rsa_mode = self.link.callback(|event: ChangeData| {
                    if let ChangeData::Select(select_element) = event {
                        MainPageMsg::EncryptionModeSelect(select_element.value())
                    } else {
                        MainPageMsg::EncryptionModeSelectIncorrect
                    }
                });
                html! {
                    <>
                        <label for="rsa-mode">{ "Choose a mode for an RSA cipher:" }</label>
                        <select name="" id="rsa-mode" onchange={onchange_rsa_mode}>
                            <option value="" selected=true disabled=true hidden=true>{ "Choose an RSA algorithm mode" }</option>
                            <option value="rsa-encrypt">{ "Encrypt" }</option>
                            <option value="rsa-decrypt">{ "Decrypt" }</option>
                            <option value="rsa-generate">{ "Generate" }</option>
                            <option value="rsa-bruteforce">{ "Bruteforce" }</option>
                        </select>
                        { self.display_adjusted_rsa_form_part() }
                        { self.display_reset_form_button() }
                    </>
                }
            },
            _ => {
                self.link.callback(|_: Event| MainPageMsg::ResetFormFields);
                html! {}
            },
        }
    }

    fn display_adjusted_form_button(&self) -> Html {
        let main_page_component = self.clone();
        match main_page_component.selected_mode {
            Some(mode) if (mode.eq(&Mode::Encode) &&
                (main_page_component.selected_cipher.eq(&Some(Cipher::Caesar)) ||
                    main_page_component.selected_cipher.eq(&Some(Cipher::Vigenere)) ||
                    main_page_component.selected_cipher.eq(&Some(Cipher::RSA)))) => {
                match main_page_component.selected_cipher {
                    Some(cipher) if (cipher.eq(&Cipher::Caesar) ||cipher.eq(&Cipher::Vigenere)) => {
                        let onclick_symmetric_encrypt = self.link.callback(|_| MainPageMsg::SymmetricEncrypt);
                        html! {
                            <button type="button" onclick={onclick_symmetric_encrypt}>{ "Encrypt" }</button>
                        }
                    },
                    Some(cipher) if cipher.eq(&Cipher::RSA) => {
                        let onclick_rsa_encrypt = self.link.callback(|_| MainPageMsg::RSAEncrypt);
                        html! {
                            <button type="button" onclick={onclick_rsa_encrypt}>{ "Encrypt" }</button>
                        }
                    },
                    _ => {
                        self.link.callback(|_: Event| MainPageMsg::ResetFormFields);
                        html! {}
                    },
                }
            },
            Some(mode) if (mode.eq(&Mode::Decode) &&
                (main_page_component.selected_cipher.eq(&Some(Cipher::Caesar)) ||
                    main_page_component.selected_cipher.eq(&Some(Cipher::Vigenere)) ||
                    main_page_component.selected_cipher.eq(&Some(Cipher::RSA)))) => {
                match main_page_component.selected_cipher {
                    Some(cipher) if (cipher.eq(&Cipher::Caesar) ||cipher.eq(&Cipher::Vigenere)) => {
                        let onclick_symmetric_decrypt = self.link.callback(|_| MainPageMsg::SymmetricDecrypt);
                        html! {
                            <button type="button" onclick={onclick_symmetric_decrypt}>{ "Decrypt" }</button>
                        }
                    },
                    Some(cipher) if cipher.eq(&Cipher::RSA) => {
                        let onclick_rsa_decrypt = self.link.callback(|_| MainPageMsg::RSADecrypt);
                        html! {
                            <button type="button" onclick={onclick_rsa_decrypt}>{ "Decrypt" }</button>
                        }
                    },
                    _ => {
                        self.link.callback(|_: Event| MainPageMsg::ResetFormFields);
                        html! {}
                    },
                }
            },
            _ => {
                self.link.callback(|_: Event| MainPageMsg::ResetFormFields);
                html! {}
            },
        }
    }

    fn display_adjusted_rsa_form_part(&self) -> Html {
        let main_page_component = self.clone();
        if main_page_component.selected_cipher.eq(&Some(Cipher::RSA)) {
            match main_page_component.selected_mode {
                Some(mode) if (mode.eq(&Mode::Encode) || mode.eq(&Mode::Decode)) => {
                    let oninput_rsa_target = self.link.callback(|event: InputData| MainPageMsg::TargetString(event.value));
                    let oninput_rsa_exponent = self.link.callback(|event: InputData| MainPageMsg::RSAExponent(event.value));
                    let oninput_rsa_modulus = self.link.callback(|event: InputData| MainPageMsg::RSAModulus(event.value));
                    html! {
                        <>
                            <label for="rsa-target">{ "Provide a target string for encryption/decryption:" }</label>
                            <input type="text" id="rsa-target" oninput={oninput_rsa_target}/>
                            <label for="rsa-exponent">{ "Provide an RSA public or private exponent for encryption/decryption:" }</label>
                            <input type="text" id="rsa-exponent" oninput={oninput_rsa_exponent}/>
                            <label for="rsa-modulus">{ "Provide an RSA modulus for encryption/decryption:" }</label>
                            <input type="text" id="rsa-modulus" oninput={oninput_rsa_modulus}/>
                            { self.display_adjusted_form_button() }
                        </>
                    }
                },
                Some(mode) if mode.eq(&Mode::Generate) => {
                    let onclick_rsa_generate = self.link.callback(|_| MainPageMsg::RSAGenerate);
                    html! {
                        <button type="button" onclick={onclick_rsa_generate}>{ "Generate" }</button>
                    }
                },
                Some(mode) if mode.eq(&Mode::Bruteforce) => {
                    let oninput_rsa_exponent = self.link.callback(|event: InputData| MainPageMsg::RSAExponent(event.value));
                    let oninput_rsa_modulus = self.link.callback(|event: InputData| MainPageMsg::RSAModulus(event.value));
                    let oninput_rsa_threads = self.link.callback(|event: InputData| MainPageMsg::RSAThreads(event.value));
                    let onclick_rsa_bruteforce = self.link.callback(|_| MainPageMsg::RSABruteforce);
                    html! {
                        <>
                            <label for="rsa-brute-exponent">{ "Provide an RSA public exponent for bruteforcing:" }</label>
                            <input type="text" id="rsa-brute-exponent" oninput={oninput_rsa_exponent}/>
                            <label for="rsa-brute-modulus">{ "Provide an RSA modulus for bruteforcing:" }</label>
                            <input type="text" id="rsa-brute-modulus" oninput={oninput_rsa_modulus}/>
                            <label for="rsa-threads">{ "Provide an optional number of threads to be used during an RSA key pair generation:" }</label>
                            <input type="text" id="rsa-threads" oninput={oninput_rsa_threads}/>
                            <button type="button" onclick={onclick_rsa_bruteforce}>{ "Bruteforce" }</button>
                        </>
                    }
                },
                _ => {
                    self.link.callback(|_: Event| MainPageMsg::ResetFormFields);
                    html! {}
                },
            }
        } else {
            self.link.callback(|_: Event| MainPageMsg::ResetFormFields);
            html! {}
        }
    }

    fn display_reset_form_button(&self) -> Html {
        let onclick_reset = self.link.callback(|_| MainPageMsg::ResetFormFields);
        html! {
            <button type="button" onclick={onclick_reset}>{ "Reset the form's fields" }</button>
        }
    }
}
