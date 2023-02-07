fn main() {
    let user1=User{
        username:String::from("neeraj"),
        emailid:String::from("neerajrp1999@gmail.com"),
        sign_in_count:64,
        active:true,
    };

    let mut user=User{
        username:String::from("neeraj"),
        emailid:String::from("neerajrp1999@gmail.com"),
        sign_in_count:64,
        active:true,
    };
    
    user.username=String::from("radhe");
    let name=user.username;
    println!("{},{}",name,user1.username);
    let funUser=build_user(String::from("Manish"),String::from("manish@gmail.com"));
    println!("{}",funUser.username);
    struct point(
        i32,i32,i32
    );
    let rect=getRectangle(34,56);
    println!("{:?}",rect);
    println!("{:#?}",rect);
    println!("rect.area():{}",rect.area());
    let n=getAreaRectangle(rect);
    
    println!("getAreaRectangle:{}",n);
    let rec=Rectangle::squre(45);
    println!("rect.area():{}",rec.area());
}
fn getRectangle(h:i64,w:i64)->Rectangle{
    Rectangle{
        height:h,
        width:w
    }
}
impl Rectangle{
    fn area(&self)->i64{
        self.height*self.width
    }
}
impl Rectangle{
    fn squre(side:i64)->Rectangle{
        Rectangle{
            height:side,
            width:side
        }
    }
}
#[derive(Debug)]
struct Rectangle{
    height:i64,
    width:i64
}
fn getAreaRectangle(rect :Rectangle)->i64{
    rect.height*rect.width
}
struct User {
    username:String,
    emailid:String,
    sign_in_count:u64,
    active:bool,
}
struct color(
    i32,i32,i32
);
fn build_user(name:String,email:String)->User{
    User{
        username:name,
        emailid:email,
        sign_in_count:1,
        active:true
    }

}