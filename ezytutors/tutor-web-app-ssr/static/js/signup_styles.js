const signinButton = document.getElementById("signup");
const signupButton = document.getElementById("signin");

function setColors(button1, button2, bg1, color1, bg2, color2) {
  // 버튼 1의 색 설정
  button1.style.backgroundColor = bg1;
  button1.style.color = color1;

  // 버튼 2의 색 설정
  button2.style.backgroundColor = bg2;
  button2.style.color = color2;
}

signinButton.addEventListener("mouseover", () => {
  // Sign in을 hover하면 Sign in은 어두운 배경, Sign up은 밝은 배경
  setColors(signinButton, signupButton, "#333", "#fff", "#fff", "#555");
});

signinButton.addEventListener("mouseout", () => {
  // 원래 상태로 되돌림
  setColors(signinButton, signupButton, "#333", "#fff", "#fff", "#555");
});

signupButton.addEventListener("mouseover", () => {
  // Sign up을 hover하면 Sign up은 어두운 배경, Sign in은 밝은 배경
  setColors(signupButton, signinButton, "#333", "#fff", "#fff", "#555");
});

signupButton.addEventListener("mouseout", () => {
  // 원래 상태로 되돌림
  setColors(signupButton, signinButton, "#fff", "#555", "#333", "#fff");
});
