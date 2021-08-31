function ajax() {
    let url = 'http://localhost:9090'
    let xhr = new XMLHttpRequest()
    xhr.open('GET', url, true)
    xhr.onreadystatechange = (e) => {
        if (e.target.readyState === 4 && e.target.status === 200) {
            console.log(e.target.responseText)
        }
    }
    xhr.send()
}

function socket() {
    let ws = new WebSocket(`ws://localhost:9090/ws/`)
    ws.onopen = function () {
        console.log('open...');
        ws.send("hello world")
    }
    ws.onclose = function () {
        console.log('close...');
    }
    ws.onmessage = function (ev) {
        console.log('message: ', ev.data);
    }
}