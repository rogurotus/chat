var xhr = new XMLHttpRequest();
xhr.open('GET', '/User', false);
xhr.send();
if (xhr.status != 200) {
    // обработать ошибку
    alert( xhr.status + ': ' + xhr.statusText ); // пример вывода: 404: Not Found
  } else {
    // вывести результат
    alert( xhr.responseText ); // responseText -- текст ответа.
  }