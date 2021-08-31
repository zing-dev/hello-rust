let url = 'http://localhost:9090'
let xhr = new XMLHttpRequest()
xhr.open('GET', url, true)
xhr.onreadystatechange = (e) => {
    if (e.target.readyState === 4 && e.target.status === 200) {
        console.log(e.target.responseText)
    }
}
xhr.send()