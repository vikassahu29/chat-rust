<template>
<div id="dashboard">
    <h4>Welcome to the Dashboard</h4>
    <div id="messages">
        <div class="message" v-for="(message,index) in messages" :key=index>
            <strong>{{message.username}}:</strong>  {{message.content}}
        </div>
        <hr/>
        <label>
            Enter Message:
            <input v-model="user_message" v-on:keyup.enter="sendMessage()" type="text"/>
        </label>
    </div>

</div>
    
    
</template>

<script>
import Helper from '../helper';

export default {
    mixins: [Helper],
    name: 'Dashboard',
    created() {
        
        this.$connect();
        this.$options.sockets.onmessage = function (data) {
            let json_data = JSON.parse(data.data);
            if (json_data["message_type"] == "chat_message") {
                this.messages.push(json_data);
            }
        }
    },
    data() {
        return {
            messages: [],
            user_message: ''
        };
    },
    methods: {
        sendMessage() {
            if (this.user_message !== "") {
                this.$socket.sendObj({
                    "message_type":"chat_message",
                    "content": this.user_message,
                    "username": this.getUsername()
                });
                this.user_message = "";
            }
        }
    }
}
</script>

<style scoped>
#messages {
    background-color:blanchedalmond;
    border: 0.05em solid brown;
    padding: 2em;
}
.message {
    margin-bottom: 0.5em;
}
</style>



