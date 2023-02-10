from flask import Flask, request, render_template, redirect, url_for, send_from_directory, jsonify, send_file
import os,json,re

ALT_URL = 'http://ph.optifine.net'
#MAIN_URL = 'http://s.optifine.net'
CAPE_REGEX = re.compile(r'http://s\.optifine\.net/capes/(.+)\.png')
capes = os.listdir('cape_file')

if not os.path.exists('users.json'):
    usersFile = open('users.json', 'w')
    usersFile.write('{}')
    usersFile.close()

usersFile = open('users.json', 'r')
users = json.load(usersFile)


def updateUsersFile(username,cape):
    users[username] = {
        'cape': cape,
        'url': os.getcwd() + '/cape_file/' + cape + '.png',
        'name': cape + '.png'
    }
    usersFile = open('users.json', 'w')
    usersFile.write(json.dumps(users, indent=4))
    usersFile.close()

app = Flask(__name__)


@app.before_request
def before_request():
    url = request.url
    print(url)
    #check if it matches the regex
    if re.match(CAPE_REGEX, url):
        username = url.split('/')[-1].split('.')[0]
        if username in users:
            return send_file(users[username]['url'])
        else:
            #send ALT_URL + '/capes/' + username + '.png' to client
            return redirect(ALT_URL + '/capes/' + username + '.png')

            
        

@app.route('/')
def index():
    return render_template('index.html', capes=[cape for cape in capes if cape.endswith('.png')])
@app.route('/cape_file/<path:path>')
def cape_file(path):
    return send_from_directory('cape_file', path)
@app.route('/submit', methods=['POST'])
def submit():
    if request.method == 'POST':
        if request.json['ign'] == '':
            return jsonify({'error': 'Please enter a username.'})
        elif request.json['cape'] == '':
            return jsonify({'error': 'Please select a cape.'})
        else:
            updateUsersFile(request.json['ign'], request.json['cape'])
            return jsonify({'success': 'Your cape has been set!'})
if __name__ == '__main__':
    app.run(debug=True, port=80, host='127.0.0.1')