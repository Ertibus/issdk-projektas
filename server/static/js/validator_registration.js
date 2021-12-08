function registration() {
    var name=document.getElementById("username").value;
    var email=document.getElementById("username").value;
    var passwd=document.getElementById("password").value;
    var cpasswd=document.getElementById("c_password").value;

    var pwd_expression = /^(?=.*?[A-Z])(?=.*?[a-z])(?=.*?[0-9])(?=.*?[#?!@$%^&*-])/;
    var letters = /^[A-Za-z]+$/;
    var filter = /^([a-zA-Z0-9_\.\-])+\@(([a-zA-Z0-9\-])+\.)+([a-zA-Z0-9]{2,4})+$/;

    if(name=='') {
      alert('Please enter you username');
      return false;

    } else if(!letters.test(name)) {
      alert('Username field can be only alphabet characters');
      return false;

    } else if(email=='') {
      alert('Please enter you email');
      return false;

    } else if(!filter.test(email)) {
      alert('Invalid email');
      return false;

    } else if(passwd=='') {
      alert('Please enter you password');
      return false;

    } else if(!pwd_expression.test(passwd)) {
      alert('Upper case, Lower case, Special character and Numeric letter are required in Password filed');
      return false;

    } else if(!passwd!=cpasswd) {
      alert('Invalid email');
      return false;

    } else if(password=='') {
      alert('Please enter you password');
      return false;

    } else {
      alert('Success!')
      return true;
    }
}
