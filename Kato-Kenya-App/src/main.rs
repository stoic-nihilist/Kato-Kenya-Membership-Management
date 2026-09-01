//mod register_member;
use std::process::ExitCode;
use eframe::{egui, Frame};
use eframe::egui::{include_image, Ui};
use image::GenericImageView;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::Page::Membership;

//#[tokio::main]
fn main() ->  eframe::Result<()> {

    let client = reqwest::Client::new();

    let icon_bytes = include_bytes!("kato-kenya.png");
    let icon_image = image::load_from_memory(icon_bytes).expect("Failed to load icon image").to_rgba8();

    let(width,height) = icon_image.dimensions();
    let icon_rgba = icon_image.into_vec();

    let icon = egui::IconData
    {
        rgba: icon_rgba,
        width,
        height,
    };





    let options = eframe::NativeOptions
    {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_resizable(false)
            .with_icon(icon),
        ..Default::default()
    };

    eframe::run_native(
        "Kato Kenya Member Management",
        options,
        Box::new(|_cc|Ok(Box::new(App::default()))),
    )

}

#[derive(Default, PartialEq)]
enum Page
{
    #[default]
    Home,
    Membership,
    Finance,
    Reports,
    Administration,

}

#[derive(Default, PartialEq)]
enum MembershipSubPage {
    #[default]
    Registration,
    AmendMember,
    ContactPerson,
    MemberDirectory,
}

#[derive(Default, PartialEq)]
enum AdminSubPage
{
    #[default]
    ManageUsers,
    AddUser,
    Login,
    Logout,

}

//#[derive(Default, PartialEq)]
//enum ManageUserSubPage {
  //  #[default]

//}

#[derive(Serialize, Deserialize, Default)]
pub struct LoginRequest
{
    pub user_id : String,
    pub password : String,
}


#[derive(Serialize, Deserialize, Default, Debug)]
pub struct LoginResponse
{
    pub token : String,
    pub valid : bool,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Claims
{
    pub sub : String,
    pub exp : usize,
}


#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Member
{
    pub member_id: String,
  //  pub membership_number: String,
    pub company_name: String,
    pub trading_name: String,
    pub company_type: String,
    pub registration_number: String,
    pub tax_pin: String,
    pub year_established: String,
    pub website: String,
    pub member_email: String,
    pub member_phone_primary: String,
    pub member_phone_secondary: String,
   // pub whatsapp_number: String,
    pub physical_address: String,
    pub postal_address: String,
    pub city: String,
    pub county_state: String,
    pub country: String,
    //pub gps_coordinate: String,
    pub company_profile: String,
    //pub number_of_staff: i32,
    pub annual_turnover: String,
    pub status: String,
    pub membership_category_id: String,
    pub joining_date: String,
   // pub expiration_date: String,
    //pub renewal_date: String,
    pub approved_by: String,
   // pub approval_date: String,
   // pub created_at: String,
  //  pub updated_at: String,

}

impl Member{

   /*
    async fn get_member(client: &Client, token: &str) -> Result<Member, Box<dyn std::error::Error>>
    {
        let member = client
        .get("http://localhost/members")
        .bearer_auth(token)
        .send()
        .await?
        .json::<Vec<Member>>()
            .await?;

        Ok(member)
    } */

    async fn get_all_members(client : &Client, token: &str) -> Result<Vec<Member>, Box<dyn std::error::Error>>
    {
        let members = client
        .get("http://localhost/members")
        .bearer_auth(token)
        .send()
        .await?
        .json::<Vec<Member>>()
        .await?;

        Ok(members)
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Contact
{
    pub contact_id:String,
    pub contact_member_id : String,
    pub contact_first_name : String,
    pub contact_last_name : String,
    pub contact_designation : String,
    pub contact_email : String,
    pub contact_phone : String,
    pub contact_national_id : String,

}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct User
{
    pub user_member_id: String,
    pub user_id: String,
    pub user_first_name: String,
    pub user_last_name: String,
    pub user_email: String,
    pub user_phone: String,
    pub user_password: String,
    pub user_password_confirm : String,
    pub user_password_hash: String,
    pub role_id: String,
    pub account_status: String,
    pub last_login: String,
    pub two_factor: bool,
    pub role_name: String,
    pub role_description: String,

}

#[derive(Clone, Debug, PartialEq)]
pub enum MessageKind
{
    Success,
    Error,
    Info,
}

#[derive(Clone, Debug)]
pub struct Message
{
    pub text: String,
    pub kind: MessageKind,
    pub timestamp: f64,
    pub duration: f64,
}

impl Message{
    pub fn success(text: impl Into<String>) -> Self
    {
        Self
        {
            text: text.into(),
            kind: MessageKind::Success,
            timestamp: 0.0,
            duration: 4.0,
        }
    }

    pub fn error(text: impl Into<String>) -> Self
    {
        Self
        {
            text: text.into(),
            kind: MessageKind::Error,
            timestamp: 0.0,
            duration: 5.0,
        }
    }

    pub fn info(text: impl Into<String>) -> Self
    {
        Self
        {
            text: text.into(),
            kind: MessageKind::Info,
            timestamp: 0.0,
            duration: 3.0,
        }
    }

    pub fn is_expired(&self, now: f64) -> bool
    {
        now - self.timestamp > self.duration
    }

}


#[derive(Default)]
pub struct App {

    message : Option<Message>,

    client : reqwest::blocking::Client,

    bg: Option<egui::TextureHandle>,

    //pages and sub-pages
    current_page: Page,
    membership_sub_page: MembershipSubPage,
    admin_sub_page: AdminSubPage,

    //members
    member : Member,
    //contact persons
   contact : Contact,
    //users
    user : User,

    //roles
    //role_name: String,
    role_description: String,

    //permissions
    //permission_id: i32,
    //permission_name: String,


    //search box parameters
    member_name_search_box : String,
    user_first_name_search_box : String,
    user_last_name_search_box : String,

    //login request and response
    login_request: LoginRequest,
    login_response: LoginResponse,

}


impl App
{

    pub fn show_message(&mut self, msg: Message)
    {
        let mut m = msg.clone();

        m.timestamp = -999.0;
        self.message = Some(msg);
    }

    pub fn render_message(&mut self, ctx: &egui::Context)
    {
        let mut message = self.message.clone();//.unwrap();
        if let Some(msg) = &mut message {
            let now = ctx.input(|i| i.time);

            if msg.timestamp == 0.0
            {
                msg.timestamp = now;
                self.message = Some(msg.clone());
            }

            if msg.is_expired(now)
            {
                self.message = None;
                return;
            }
            let color = match msg.kind
            {
                MessageKind::Success => egui::Color32::from_rgb(76, 175, 80),    // Green
                MessageKind::Error => egui::Color32::from_rgb(244, 67, 54),      // Red
                MessageKind::Info => egui::Color32::from_rgb(33, 150, 243),      // Blue
            };
            let icon = match msg.kind
            {
                MessageKind::Success => "✓",
                MessageKind::Error => "X",
                MessageKind::Info => "ℹ",
            };
            egui::TopBottomPanel::top("message_panel")
                .exact_height(50.0)
                .frame(egui::Frame::none().fill(color).inner_margin(10.0))
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading(
                            egui::RichText::new(icon)
                                .size(24.0)
                                .color(egui::Color32::WHITE)
                        );
                        ui.label(
                            egui::RichText::new(&msg.text)
                                .size(16.0)
                                .color(egui::Color32::WHITE)
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("X").clicked()
                            {
                                self.message = None;
                            }
                        });
                    });
                });
        }
    }

    fn add_member_blocking(client: &reqwest::blocking::Client, token: &str, member: &Member)
        -> Result<Member, Box<dyn std::error::Error>>
    {
        let res = client
            .post("http://localhost:3000/members")
            .bearer_auth(&token)
            .json(&member)
            .send()?;

        let status = res.status();
        let body = res.text()?;


        println!("Status: {}", status);
        println!("Response: {}", body);

        if !status.is_success()
        {
            return Err(format!("Failed to add user: {}", body).into());
        }
        else {
            println!("Member added successfully.");
        }


        let member_res: Member = serde_json::from_str(&body)?;
        Ok(member_res)


    }


    fn add_member_button
    (
        &mut self,
        ui : &mut egui::Ui,
        text : &str,
    )

    {
        let button = egui::Button::new(egui::RichText::new(text).size(26.0)).fill( egui::Color32::from_rgb(52, 73, 30)).stroke(egui::Stroke::NONE)
            .rounding(0.0);

        if ui.add_sized([500.0, 50.0], button).clicked()
        {
            let client = self.client.clone();
            let token = self.login_response.token.clone();
            let member = self.member.clone();

            match App::add_member_blocking( &client, &token, &member)
            {
                Ok(res) =>
                    {
                        self.show_message(Message::success("Member added successfully"));
                        println!("Member added: {:?}", res);
                        self.member = Member::default();
                        ui.memory_mut(|mem| mem.data.clear());
                    }
                Err(e) =>
                    {
                        self.show_message(Message::error(format!("Error: {}", e)));
                        eprintln!("Error: {}", e);
                    }
            }



        }
    }


    fn add_contact_blocking(client: &reqwest::blocking::Client, token: &str, contact: &Contact)
    -> Result<Contact, Box<dyn std::error::Error>>
    {
        let res = client
            .post("http://localhost:3000/contact_persons")
        .bearer_auth(&token)
        .json(&contact)
        .send()?;

        let status = res.status();

        let body = res.text()?;

        println!("Status: {}", status);
        println!("Response: {}", body);

        if !status.is_success()
        {
            return Err(format!("Failed to add contact: {}", body).into());
        }

        let contact_res: Contact = serde_json::from_str(&body)?;

        Ok(contact_res)
    }

    fn add_contact_button
    (
        &mut self,
        ui : &mut egui::Ui,
        text : &str,
    )
    {
        let button = egui::Button::new(egui::RichText::new(text).size(26.0)).fill( egui::Color32::from_rgb(52, 73, 30)).stroke(egui::Stroke::NONE)
            .rounding(0.0);

        if ui.add_sized([500.0, 50.0], button).clicked()
        {
            let client = self.client.clone();
            let token = self.login_response.token.clone();
            let contact = self.contact.clone();

            match App::add_contact_blocking( &client, &token, &contact)
            {
                Ok(res) =>
                    {

                        self.show_message(Message::success("Contact added successfully"));

                        println!("Contact: {:?}, added", res);
                        self.contact = Contact::default();
                    }
                Err(e) =>
                    {
                        self.show_message(Message::error(format!("Error: {}", e)));
                        eprintln!("Error: {}", e);
                    }
            }
        }

    }



    fn add_user_blocking( client: &reqwest::blocking::Client, token: &str, user: &User)
        -> Result<User, Box<dyn std::error::Error>>
    {


       // self.user.user_first_name = user.user_first_name.clone();

        let res = client
            .post("http://localhost:3000/users")
            .bearer_auth(&token)
            .json(&user)
            .send()?;

        let status = res.status();

        let body = res.text()?;

        println!("Status: {}", status);
        println!("Response: {}", body);

        if !status.is_success()
        {
            return Err(format!("Failed to add user: {}", body).into());
        }


        else
        {
            println!("User successfully added");
        }

        let user_res: User = serde_json::from_str(&body)?;
        Ok(user_res)
    }



    fn login_blocking(&mut self, client: &reqwest::blocking::Client, username: &str, password: &str) -> Result<LoginResponse, Box<dyn std::error::Error>>
    {
        let res = client
            .post("http://localhost:3000/login")
            .json(&LoginRequest
            {
                user_id: username.into(),
                password : password.into()
            })
            .send()?;

        let status = res.status();
        let body = res.text()?;

        println!("Status: {}", status);

        println!("Response body: {}", body);

        if !status.is_success()
        {
            return Err(format!("Login failed: {}", body).into());
        } else {
            println!("Login succeeded");
        }

        let login_res: LoginResponse = serde_json::from_str(&body)?;

        self.login_response.token = login_res.token.clone();
        // .json::<LoginResponse>()?;

        Ok(login_res)
    }



    fn logout(&mut self)
    {
        self.login_response.token = String::new();
        self.login_response.valid = false;
        println!("Logged out");
    }

    fn match_error
    (
        &mut self,
        ui: &mut Ui,
        text: &str,
    )

    {
        ui.label(egui::RichText::new(text).size(24.0));
    }


    fn menu_button(
        &mut self,
        ui: &mut egui::Ui,
        text: &str,
        page: Page,

    )
    {
        let selected = self.current_page == page;
        let button = egui::Button::new(egui::RichText::new(text).size(26.0)).fill(
            //egui::RichText::new(text).size(18.0)
            if selected
            {
                egui::Color32::from_rgb(52, 73, 30)
            } else { egui::Color32::from_rgb(40, 40, 40) }
        )
            .stroke(egui::Stroke::NONE)
            .rounding(0.0);

        if ui.add_sized(
            [200.0, 50.0],
            //[ui.available_height(), 50.0],
            button,
        ).clicked() {
            self.current_page = page;
        }
    }

    fn exit_button
    (
        &mut self,
        ui: &mut Ui,
        text: &str,
    )

    {
        let button = egui::Button::new(egui::RichText::new(text).size(26.0)).fill(
            //egui::RichText::new(text).size(18.0)

               egui::Color32::from_rgb(40, 40, 40)
        )
            .stroke(egui::Stroke::NONE)
            .rounding(0.0);

        if ui.add_sized(
            [200.0, 50.0],
            //[ui.available_height(), 50.0],
            button,
        ).clicked() {
            ExitCode::SUCCESS;
        }
    }



    fn add_user_button(
        &mut self,
        ui: &mut egui::Ui,
        text: &str,
    )

    {
        let button = egui::Button::new(egui::RichText::new(text).size(26.0)).fill(egui::Color32::from_rgb(52, 73, 30)).stroke(egui::Stroke::NONE).rounding(0.0);

        if ui.add_sized([500.0,50.0], button).clicked() {
            let client = self.client.clone();
            let token = self.login_response.token.clone();
           // let token_debug = self.login_response.token.clone();
            let user = self.user.clone();
           // let password = self.user.user_password.clone();
            // let confirm_password = self.user.user_password_confirm.clone();
            if !token.is_empty()
            {
             //   println!("Token: {}", {token_debug});
            } else
            {
                println!("No token");
            }


                match App::add_user_blocking( &client, &token, &user)
                {
                    Ok(res) =>
                        {
                            self.show_message(Message::success("User added successfully"));
                            println!("User added: {:?}", res);
                            self.user = User::default();
                            ui.memory_mut(|mem| mem.data.clear());
                        }
                    Err(e) =>
                        {
                            self.show_message(Message::error(format!("Error: {}", e)));
                            eprintln!("Error: {}", e);
                        }
                }




        }
    }


    fn session_button
    (
        &mut self,
        ui: &mut egui::Ui,
        text: &str,
        sub_page: AdminSubPage,
    )
    {
        match text
        {
            "Log In" =>
                {
                    let button = egui::Button::new(egui::RichText::new(text).size(26.0)).fill(egui::Color32::from_rgb(52, 73, 30)).stroke(egui::Stroke::NONE).rounding(0.0);

                    if ui.add_sized([500.0,50.0], button).clicked() {
                        let client = self.client.clone();
                        let token = self.login_response.token.clone();
                        let user = self.user.clone();

                        match App::login_blocking(self, &client, &user.user_id, &user.user_password)
                        {
                            Ok(res) =>
                                {
                                    self.show_message(Message::success("Logged in successfully."));
                                    self.user = User::default();
                                    self.admin_sub_page = sub_page;
                                    ui.memory_mut(|mem| mem.data.clear());

                                }
                            Err(e) =>
                                {
                                    self.show_message(Message::error(format!("Error: {}", e)));
                                    eprintln!("Error: {}", e);
                                }
                        }
                    }
                }

            "Log Out" =>
                {

                    let button = egui::Button::new(egui::RichText::new(text).size(26.0)).fill(egui::Color32::from_rgb(52,73,30)).stroke(egui::Stroke::NONE).rounding(0.0);
                    if ui.add_sized([500.0, 50.0], button).clicked() {
                        self.logout();
                        self.show_message(Message::info("Logged out"));
                        self.login_response.valid = false;
                        self.admin_sub_page = sub_page;
                    }
                }

            _ =>
                {

                }
        }


    }


    fn member_button(
        &mut self,
        ui: &mut egui::Ui,
        text: &str,
        sub_page: MembershipSubPage,
    )
    {
        let selected = self.membership_sub_page == sub_page;
        let button = egui::Button::new(egui::RichText::new(text).size(26.0)).fill(
            //egui::RichText::new(text).size(18.0)
            if selected
            {
                egui::Color32::from_rgb(52, 73, 30)
            } else { egui::Color32::from_rgb(40, 40, 40) }
        )
            .stroke(egui::Stroke::NONE)
            .rounding(0.0);

        if ui.add_sized(
            [500.0, 50.0],
            //[ui.available_height(), 50.0],
            button,
        ).clicked() {
            self.membership_sub_page = sub_page;
        }
    }

    fn admin_button(
        &mut self,
        ui: &mut egui::Ui,
        text: &str,
        sub_page: AdminSubPage,
    )
    {
        let selected = self.admin_sub_page == sub_page;
        let button = egui::Button::new(egui::RichText::new(text).size(26.0)).fill(
            if selected
            {
                egui::Color32::from_rgb(52, 73, 30)
            } else { egui::Color32::from_rgb(40, 40, 40) }
        )
            .stroke(egui::Stroke::NONE)
            .rounding(0.0);

        if ui.add_sized(
            [500.0, 50.0],
            button,
        ).clicked()
        {
            self.admin_sub_page = sub_page;
        }
    }
}

impl eframe::App for App {



    fn ui(&mut self, ui: &mut Ui, frame: &mut Frame) {
            //this does literally nothing but the compiler wouldn't stop bitching about it.
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        self.render_message(ctx);


        //setting up the KATO bg logo
        if self.bg.is_none() {
            let image_bytes = include_bytes!("kato_logo_bg.png");

            let image = image::load_from_memory(image_bytes).expect("failed to load image").to_rgba8();

            let size = [image.width() as _, image.height() as _];
            let pixels = image.into_vec();

            let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);


            let texture = ctx.load_texture("bg", color_image, egui::TextureOptions::default());

            self.bg = Some(texture);
        }




        ctx.set_visuals(egui::Visuals
        {
            panel_fill: egui::Color32::from_rgb(255,255,237),
            //,
            extreme_bg_color: egui::Color32::from_rgb(255,255,237),
                ..egui::Visuals::dark()
        });


        egui::SidePanel::right("menu_panel").min_width(200.0).show(ctx, |ui|
        {
            ui.spacing_mut().item_spacing.y = 0.0;

            ui.heading(egui::RichText::new("Dashboard").size(32.0));
            ui.separator();

            self.menu_button(ui, "Home", Page::Home);
            //ui.separator();
            self.menu_button(ui, "Membership", Page::Membership);
            //ui.separator();
            self.menu_button(ui, "Finance", Page::Finance);
            //ui.separator();
            self.menu_button(ui, "Reports", Page::Reports);
            self.menu_button(ui, "Administration", Page::Administration);
          //  self.exit_button(ui, "Exit");


        });

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.vertical(|ui|{




                match self.current_page
                {
                    Page::Home =>
                        {
                            ui.heading(egui::RichText::new("Kato Kenya Member Management System").size(36.0));
                            ui.label(egui::RichText::new("\nWelcome to the Membership Management System Application").size(28.0));

                            if let Some(tex) = &self.bg {
                            ui.image(tex);
                        }
                          //  ui.label("Dashboard");
                            }
                    Page::Membership =>
                        {
                            egui::CentralPanel::default().show(ctx, |ui| {
                                ui.heading(egui::RichText::new("Membership Management").size(32.0));

                                ui.horizontal(|ui|
                                    {
                                        self.member_button(ui, "Member Registration", MembershipSubPage::Registration);
                                        //self.member_button(ui, "Reports", Page::Reports);
                                        self.member_button(ui, "Search Members", MembershipSubPage::AmendMember);

                                    });

                                ui.horizontal(|ui| {
                                    self.member_button(ui, "Add Contact Persons", MembershipSubPage::ContactPerson);
                                    self.member_button(ui, "Member Directory", MembershipSubPage::MemberDirectory);
                                });

                                match self.membership_sub_page
                                {
                                    MembershipSubPage::Registration =>
                                        {
                                           // egui::CentralPanel::default().show(ui, |ui| {

                                            ui.heading(egui::RichText::new("Member Registration").size(32.0));
                                            egui::ScrollArea::vertical().max_height(600.0).show(ui, |ui| {

                                                ui.label(egui::RichText::new("Enter Company Name:").size(20.0));
                                                ui.add_sized([400.0,20.0],egui::TextEdit::singleline(&mut self.member.company_name).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                ui.scope(|ui| {
                                                    ui.set_width(500.0);
                                                    ui.separator();
                                                });
                                                ui.label(egui::RichText::new("Enter Trading Name (optional):").size(20.0));
                                                ui.add_sized([400.0,20.0],egui::TextEdit::singleline(&mut self.member.trading_name).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                ui.scope(|ui| {
                                                    ui.set_width(500.0);
                                                    ui.separator();
                                                });
                                                ui.label(egui::RichText::new("Enter Company Type:").size(20.0));
                                                ui.add_sized([400.0,20.0],egui::TextEdit::singleline(&mut self.member.company_type).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                ui.scope(|ui| {
                                                    ui.set_width(500.0);
                                                    ui.separator();
                                                });
                                                ui.label(egui::RichText::new("Enter Government Registration Number:").size(20.0));
                                                ui.add_sized([400.0, 20.0],egui::TextEdit::singleline(&mut self.member.registration_number).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                ui.scope(|ui| {
                                                    ui.set_width(500.0);
                                                    ui.separator();
                                                });
                                                ui.label(egui::RichText::new("Enter Company Tax/VAT Pin Number:").size(20.0));
                                                ui.add_sized([400.0, 20.0],egui::TextEdit::singleline(&mut self.member.tax_pin).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                ui.scope(|ui| {
                                                    ui.set_width(500.0);
                                                    ui.separator();
                                                });
                                                ui.label(egui::RichText::new("Enter Year Established:").size(20.0));
                                                ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.member.year_established).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                ui.scope(|ui| {
                                                    ui.set_width(500.0);
                                                    ui.separator();
                                                });
                                                ui.label(egui::RichText::new("Enter Company Website:").size(20.0));
                                                ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.member.website).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                ui.scope(|ui| {
                                                    ui.set_width(500.0);
                                                    ui.separator();
                                                });
                                                ui.label(egui::RichText::new("Enter Company Email:").size(20.0));
                                                ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.member.member_email).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                ui.scope(|ui| {
                                                    ui.set_width(500.0);
                                                    ui.separator();
                                                });
                                                ui.label(egui::RichText::new("Enter Primary Phone Number:").size(20.0));
                                                ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.member.member_phone_primary).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                ui.scope(|ui| {
                                                    ui.set_width(500.0);
                                                    ui.separator();
                                                });
                                                ui.label(egui::RichText::new("Enter Secondary Phone Number (optional):").size(20.0));
                                                ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.member.member_phone_secondary).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                ui.scope(|ui| {
                                                    ui.set_width(500.0);
                                                    ui.separator();
                                                });
                                                ui.label(egui::RichText::new("Enter Company Physical Address:").size(20.0));
                                                ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.member.physical_address).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                ui.scope(|ui| {
                                                    ui.set_width(500.0);
                                                    ui.separator();
                                                });
                                                ui.label(egui::RichText::new("Enter Company Postal Address:").size(20.0));
                                                ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.member.postal_address).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                ui.scope(|ui| {
                                                    ui.set_width(500.0);
                                                    ui.separator();
                                                });
                                                ui.label(egui::RichText::new("Enter Company City:").size(20.0));
                                                ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.member.city).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                ui.scope(|ui| {
                                                    ui.set_width(500.0);
                                                    ui.separator();
                                                });
                                                ui.label(egui::RichText::new("Enter Company County/State:").size(20.0));
                                                ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.member.county_state).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                ui.scope(|ui| {
                                                    ui.set_width(500.0);
                                                    ui.separator();
                                                });
                                                ui.label(egui::RichText::new("Enter Company Country:").size(20.0));
                                                ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.member.country).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                ui.scope(|ui| {
                                                    ui.set_width(500.0);
                                                    ui.separator();
                                                });

                                                ui.label(egui::RichText::new("Enter Company Profile/Description (optional):").size(20.0));
                                                ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.member.company_profile).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                ui.scope(|ui| {
                                                    ui.set_width(500.0);
                                                    ui.separator();
                                                });

                                                ui.label(egui::RichText::new("Enter Annual Turnover:").size(20.0));
                                                ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.member.annual_turnover).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                ui.scope(|ui| {
                                                    ui.set_width(500.0);
                                                    ui.separator();
                                                });
                                                ui.label(egui::RichText::new("Enter Membership Status:").size(20.0));
                                                ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.member.status).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));//.font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                ui.scope(|ui| {
                                                    ui.set_width(500.0);
                                                    ui.separator();
                                                });
                                                ui.label(egui::RichText::new("Enter Membership Category ID:").size(20.0));
                                                ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.member.membership_category_id).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                ui.scope(|ui| {
                                                    ui.set_width(500.0);
                                                    ui.separator();
                                                });
                                                ui.label(egui::RichText::new("Enter Joining Date (YYYY-MM-DD):").size(20.0));
                                                ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.member.joining_date).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                ui.scope(|ui| {
                                                    ui.set_width(500.0);
                                                    ui.separator();
                                                });

                                                ui.scope(|ui| {
                                                    ui.set_width(500.0);
                                                    ui.separator();
                                                });
                                              //  ui.label(egui::RichText::new("Enter Membership Renewal Date (YYYY-MM-DD):").size(20.0));
                                              //  ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.renewal_date));
                                                ui.label(egui::RichText::new("Enter Approved By (Admin User ID):").size(20.0));
                                                ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.member.approved_by).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));

                                            //ui.label(format!("Current page:")); //{:?}", self.current_page));


                                                self.add_member_button(ui, "Add Member");

                                           });
                                        //});
                                }
                                   MembershipSubPage::AmendMember =>
                                       {
                                           // egui::CentralPanel::default().show(ui, |ui| {
                                           ui.heading(egui::RichText::new("Search for Member").size(32.0));


                                           //search box

                                           egui::ScrollArea::vertical().max_height(600.0).show(ui, |_ui| {

                                           });
                                      // });

                                }

                                    MembershipSubPage::ContactPerson =>
                                        {
                                            ui.heading(egui::RichText::new("Add Member Contact Person").size(32.0));

                                            egui::ScrollArea::vertical().max_height(600.0).show(ui, |ui|
                                                {
                                                    ui.label(egui::RichText::new("Enter Member Company ID: ").size(20.0));
                                                    ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.contact.contact_member_id).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                    ui.scope(|ui|
                                                        {
                                                            ui.set_width(500.0);
                                                            ui.separator();
                                                        });

                                                    ui.label(egui::RichText::new("Enter First Name:").size(20.0));
                                                    ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.contact.contact_first_name).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                    ui.scope(|ui|
                                                        {
                                                            ui.set_width(500.0);
                                                            ui.separator();
                                                        });

                                                    ui.label(egui::RichText::new("Enter Last Name:").size(20.0));
                                                    ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.contact.contact_last_name).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                    ui.scope(|ui|
                                                        {
                                                            ui.set_width(500.0);
                                                            ui.separator();
                                                        });

                                                    ui.label(egui::RichText::new("Enter Designation:").size(20.0));
                                                    ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.contact.contact_designation).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                    ui.scope(|ui|
                                                        {
                                                           ui.set_width(500.0);
                                                            ui.separator();
                                                        });

                                                    ui.label(egui::RichText::new("Enter Email:").size(20.0));
                                                ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.contact.contact_email).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                    ui.scope(|ui|
                                                    {
                                                       ui.set_width(500.0);
                                                        ui.separator();
                                                    });

                                                    ui.label(egui::RichText::new("Enter Phone:").size(20.0));
                                                    ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.contact.contact_phone).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                    ui.scope(|ui|
                                                        {
                                                            ui.set_width(500.0);
                                                            ui.separator();
                                                        });

                                                    ui.label(egui::RichText::new("Enter National ID:").size(20.0));
                                                    ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.contact.contact_national_id).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                    ui.scope(|ui|

                                                        {
                                                           ui.set_width(500.0);
                                                            ui.separator();
                                                        });


                                                    self.add_contact_button(ui, "Add Contact Person");


                                                });
                                        }

                                    MembershipSubPage::MemberDirectory =>
                                        {
                                            ui.heading(egui::RichText::new("View Member Directory").size(32.0));

                                        }
                                    _ =>
                                        {

                                        }
                                    }
                            });
                        }

                    Page::Finance =>
                        {ui.heading(egui::RichText::new("Finance and Billing").size(32.0));

                        //Renewal

                            // Membership Payments

                            //Invoices Table

                        }
                    Page::Reports =>
                        {ui.heading(egui::RichText::new("Reports and Analytics").size(32.0));
                        //Documents
                        //Licenses

                        }
                    Page::Administration =>
                        {ui.heading(egui::RichText::new("Administration and User Management").size(32.0));



                            ui.horizontal(|ui|
                                {
                                    self.admin_button(ui, "Manage Users", AdminSubPage::ManageUsers);
                                    self.admin_button(ui, "Add Users", AdminSubPage::AddUser);
                                });

                            ui.horizontal(|ui|
                            {
                                if self.login_response.valid == false
                                {
                                    self.admin_button(ui, "Session", AdminSubPage::Login);
                                }
                                else
                                {
                                    self.admin_button(ui, "Session", AdminSubPage::Logout);
                                }

                            });

                           match self.admin_sub_page
                           {

                               AdminSubPage::Login =>
                                   {

                                               ui.heading(egui::RichText::new("Log In").size(32.0));
                                               ui.scope(|ui|
                                                   {
                                                       ui.set_width(500.0);
                                                       ui.separator();
                                                   });

                                               egui::ScrollArea::vertical().max_height(600.0).show(ui, |ui|
                                                   {
                                                       ui.label(egui::RichText::new("Enter User ID:").size(26.0));
                                                       ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.user.user_id).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                       ui.scope(|ui|
                                                           {
                                                               ui.set_width(500.0);
                                                               ui.separator();
                                                           });
                                                       ui.label(egui::RichText::new("Enter Password:").size(26.0));
                                                       ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.user.user_password).password(true).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                                       ui.scope(|ui|
                                                           {
                                                               ui.set_width(500.0);
                                                               ui.separator();
                                                           });

                                                       self.session_button(ui,"Log In",AdminSubPage::Logout);


                                                   });





                                   }

                               AdminSubPage::Logout => {
                                  // println!("Logged in as: {}", self.user.user_first_name);
                                   ui.label(egui::RichText::new("Logged in").size(26.0));
                                   self.session_button(ui, "Log Out", AdminSubPage::Login);
                               }


                               AdminSubPage::AddUser =>
                                   {
                                       ui.heading(egui::RichText::new("Add Users").size(32.0));

                                       egui::ScrollArea::vertical().max_height(600.0).show(ui, |ui| {
/*
                                          ui.label(egui::RichText::new("Enter Member ID (if applicable):").size(20.0));
                                           ui.add_sized([400.0,20.0], (egui::TextEdit::singleline(&mut self.user.user_member_id).font(egui::FontId::new(26.0, egui::FontFamily::Proportional))));
                                           ui.scope(|ui| {
                                               ui.set_width(500.0);
                                               ui.separator();
                                           });
*/
                                           ui.label(egui::RichText::new("Enter First Name:").size(20.0));
                                           ui.add_sized([400.0, 20.0], egui::TextEdit::singleline(&mut self.user.user_first_name).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                           ui.scope(|ui| {
                                               ui.set_width(500.0);
                                               ui.separator();
                                           });

                                           ui.label(egui::RichText::new("Enter Last Name:").size(20.0));
                                           ui.add_sized([400.0, 20.0], egui::TextEdit::singleline((&mut self.user.user_last_name)).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                           ui.scope(|ui| {
                                               ui.set_width(500.0);
                                               ui.separator();
                                           });
                                           ui.label(egui::RichText::new("Enter User Email:").size(20.0));
                                           ui.add_sized([400.0, 20.0], egui::TextEdit::singleline(&mut self.user.user_email).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                           ui.scope(|ui| {
                                               ui.set_width(500.0);
                                               ui.separator();
                                           });

                                           ui.label(egui::RichText::new("Enter User Phone:").size(20.0));
                                           ui.add_sized([400.0,20.0], egui::TextEdit::singleline(&mut self.user.user_phone).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                           ui.scope(|ui| {
                                               ui.set_width(500.0);
                                               ui.separator();
                                           });

                                           ui.label(egui::RichText::new("Enter User Password:").size(20.0));
                                           ui.add_sized([400.0, 20.0], egui::TextEdit::singleline(&mut self.user.user_password).password(true).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                           ui.scope(|ui| {
                                               ui.set_width(500.0);
                                               ui.separator();
                                           });

                                           ui.label(egui::RichText::new("Confirm User Password:").size(20.0));
                                           ui.add_sized([400.0, 20.0], egui::TextEdit::singleline(&mut self.user.user_password_confirm).password(true).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                           ui.scope(|ui| {
                                               ui.set_width(500.0);
                                               ui.separator();
                                           });
                                           ui.scope(|ui| {
                                               ui.set_width(500.0);
                                               ui.separator();
                                           });

                                           ui.label(egui::RichText::new("Enter Role: ").size(20.0));
                                           ui.label(egui::RichText::new("1:Super Admin, 2:Finance Officer,").size(20.0));
                                           ui.label(egui::RichText::new("3:Membership Officer, 4:Read Only").size(20.0));
                                           ui.add_sized([400.0, 20.0], egui::TextEdit::singleline(&mut self.user.role_id).font(egui::FontId::new(26.0, egui::FontFamily::Proportional)));
                                           ui.scope(|ui| {
                                               ui.set_width(500.0);
                                               ui.separator();
                                           });

                                           self.add_user_button(ui, "Add User");

                                       });

                                   }

                               AdminSubPage::ManageUsers =>
                                   {
                                       ui.heading(egui::RichText::new("Manage Users").size(32.0));

                                       //search box

                                       //manage roles and permissions
                                   }

                           }


                        }
                    _ =>
                        {}
                }


              //  ui.heading("Main Screen");
            });

        });


      /*  egui::CentralPanel::default().show(ctx, |ui| {


                let rect = ui.max_rect();

                if let Some(texture) = &self.bg
                {
                   ui.image(texture);
                }

           // ui.vertical(|ui| {

                ui.heading("Kato Kenya Member Management System\n\n");
                ui.label("Refer to the Menu icon on the left for more actions.");
         //   });

        });*/



    }
    }

