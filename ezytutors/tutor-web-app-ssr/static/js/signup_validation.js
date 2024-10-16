const form = document.querySelector("#member");
const btnSubmit = form.querySelector("#signup");

form.addEventListener("submit", function (e) {
  if (!isPassword("password", 7)) e.preventDefault();
  if (!isRePassword("confirmation")) e.preventDefault();
});

function isPassword(name, len) {
  const input = form.querySelector(`[name=${name}]`);
  const password = input.value;
  var pattern = /[`~!@#$%^&*|\\\'\";:\/?]/gi;

  if (password.length > len && pattern.test(password) == true) {
    const errMsgs = input.closest("td").querySelectorAll("p");
    if (errMsgs.length > 0) input.closest("td").querySelector("p").remove();

    return true;
  } else {
    const errMsgs = input.closest("td").querySelectorAll("p");
    if (errMsgs.length > 0) input.closest("td").querySelector("p").remove();

    const errMsg = document.createElement("p");
    errMsg.append(
      `영어, 숫자, 특수기호를 조합하여 ${len + 1}글자 이상 입력하세요.`
    );
    input.closest("td").append(errMsg);

    return false;
  }
}

function isRePassword(name) {
  const input = form.querySelector(`[name=${name}]`);
  var p1 = document.getElementById("password").value;
  var p2 = document.getElementById("confirmation").value;
  if (p1 != p2) {
    const errMsgs = input.closest("td").querySelectorAll("p");
    if (errMsgs.length > 0) input.closest("td").querySelector("p").remove();
    const errMsg = document.createElement("p");
    errMsg.append("비밀번호가 일치 하지 않습니다");
    input.closest("td").append(errMsg);
    return false;
  } else {
    const errMsgs = input.closest("td").querySelectorAll("p");
    if (errMsgs.length > 0) input.closest("td").querySelector("p").remove();

    return true;
  }
}
