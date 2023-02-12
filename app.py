from flask import Flask, request, render_template, redirect, url_for, send_from_directory, jsonify, send_file
import os,json,re,requests

ALT_URL = 'http://ph.optifine.net'
LUNAR_ALT_URL = 'http://ph.lunarclientcdn.com'
#MAIN_URL = 'http://s.optifine.net'
CAPE_REGEX = re.compile(r'http://s\.optifine\.net/capes/(.+)\.png')
LUNAR_REGEX = re.compile(r'http://s-optifine\.lunarclientcdn\.com/capes/(.+)\.png')
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

def getCapeImage(url): 
    response = requests.get(url)
        # Check if the request was successful
    if response.status_code == 200:
            # If the request was successful, send the image data to the client
        return response.content, 200, { 'Content-Type': 'image/png' }
    else:
            # If the request was unsuccessful, return an error response
        return 'Error retrieving cape', 404

app = Flask(__name__)
@app.before_request
def before_request():
    url = request.url
    print(url)

@app.route('/capes/<username>.png')
def cape(username):
    if username in users:
        return send_file(users[username]['url'])
    else:
        #send ALT_URL + '/capes/' + username + '.png' to client
        cape_url = ALT_URL + '/capes/' + username + '.png'
        #send cape_url to client without redirecting
        return getCapeImage(cape_url)

        

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
# @app.route('/users/<username>')
# def user(username):
#     if username == 'PilBerry':

if __name__ == '__main__':
    app.run(debug=True, port=80, host='127.0.0.1')