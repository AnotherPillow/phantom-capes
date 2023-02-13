const capedropDown = _ => {
const capedropDown = document.createElement('div');
capedropDown.id = 'capedropDown';
capedropDown.style = 'position: absolute; top: 0; left: 0; width: 100%; height: 100%; background-color: rgba(0, 0, 0, 0.5);';

const capedropDownContent = document.createElement('div');
capedropDownContent.style = 'position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); background-color: #191919; border: 1px solid rgb(124, 164, 45); border-radius: 5px; padding: 10px;';

const capedropDownContentText = document.createElement('p');
capedropDownContentText.innerText = 'Choose a cape';
capedropDownContent.appendChild(capedropDownContentText);

const capedropDownContentCapes = document.createElement('div');
capedropDownContentCapes.style = 'display: flex; flex-wrap: wrap; justify-content: center;';
for (let i = 0; i < capes.length; i++) {
    const capedropDownContentCapesCape = document.createElement('div');
    capedropDownContentCapesCape.style = 'width: 100px; height: 100px; background-color: rgb(124, 164, 45); border: 1px solid rgb(124, 164, 45); border-radius: 5px; margin: 5px;';
    capedropDownContentCapesCape.onclick = _ => {
        document.getElementById('capedropDown').remove();
        document.getElementById('capeOuter').style = 'display: block;';
        let capeNameEl = document.getElementById('capeName')
        capeNameEl.innerText = capes[i].name;
        capeNameEl.href = capes[i].path;
        
    }
    const capedropDownContentCapesCapeImage = document.createElement('img');
    capedropDownContentCapesCapeImage.src = capes[i].path;
    capedropDownContentCapesCape.appendChild(capedropDownContentCapesCapeImage);
    capedropDownContentCapes.appendChild(capedropDownContentCapesCape);
}

capedropDownContent.appendChild(capedropDownContentCapes);
capedropDown.appendChild(capedropDownContent);
document.body.appendChild(capedropDown);
}

const submitCape = _ => {
let ign = document.getElementById('ign').value;
let cape = document.getElementById('capeName').innerText;
if (ign == '' || cape == 'null') {
    alert('Please fill out all fields');
    return;
}
fetch('/submit', {
    method: 'POST',
    headers: {
        'Content-Type': 'application/json'
    },
    body: JSON.stringify({
        ign: ign,
        cape: cape
    })
}).then(res => {
    if (res.status == 200) {
        alert('Cape submitted successfully');
    } else {
        res.json().then(data => {
            alert(data.error);
        })
    }
})
};