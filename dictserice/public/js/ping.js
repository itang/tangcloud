//use strict;;

var ping = new Vue({
    el: "#ping",
    data: {
        pingResult: ""
    }
});

// client
var client =
    (function () {
        function ping(callback) {
            /*var xhttp = new XMLHttpRequest();
            xhttp.onreadystatechange = function () {
                if (this.readyState == 4 && this.status == 200) {
                    callback(JSON.parse(this.responseText));
                }
            };
            xhttp.open("GET", "/ping", true);
            xhttp.send();*/

            axios.get('/ping')
                .then(function (response) {
                    console.log(response);
                    callback(response.data);
                })
                .catch(function (error) {
                    console.log(error);
                });
        }

        return { ping: ping };
    })();

// init
(function () {
    client.ping(ret => ping.pingResult = ret);
})();





/*

  axios({
     method: 'post',
     url: config.SERVER_URL + 'getData',
     data: { id: '1234' },
     headers: {
       'Content-Type': 'application/json'
    }
});

*/































var app1 = new Vue({
    el: "#app-1",
    data: {
        message: `Hello World: ${new Date()}`
    },
    created: function () {
        console.log('app created: a is :' + this.message);
    },
    mounted: function () {
        console.log("app mounted");
    },
    updated: function () {
        console.log("app updated");
    },
    destroyed: function () {
        console.log("app destroyed");
    }
});

var app2 = new Vue({
    el: '#app-2',
    data: {
        id: 'inspect-me'
    }
});

var app3 = new Vue({
    el: '#app-3',
    data: {
        seen: true
    }
});

var app4 = new Vue({
    el: "#app-4",
    data: {
        todos: [
            { text: 'Learn JavaScript' },
            { text: 'Learn Vue' },
            { text: 'Build something awesome' }
        ]
    }
});

var app5 = new Vue({
    el: '#app-5',
    data: {
        message: 'Hello Vue.js'
    },
    methods: {
        reverseMessage: function () {
            this.message = this.message.split('').reverse().join('')
        }
    }
});

var app6 = new Vue({
    el: '#app-6',
    data: {
        message: 'Hello Vue!'
    }
});

Vue.component('todo1', {
    template: '<li>This is a todo</li>'
});

Vue.component('todo', {
    props: ['todo'],
    template: '<li>{{ todo.text }}</li>'
});

var app7 = new Vue({
    el: '#app-7',
    data: {
        todos: [
            { text: 'Learn JavaScript' },
            { text: 'Learn Vue' },
            { text: 'Build something awesome' }
        ]
    }
});