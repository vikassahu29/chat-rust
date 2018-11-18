<template>
    <div class="Login">
        
        <form @submit.prevent="handleSubmit">
            <div class="error">{{error}}</div>
            <label>
                Username:
                <input type="text" v-model="user.username" required=true/>
            </label>
            <label>
                Password:
                <input type="password" v-model="user.password" required=true/>
            </label>
            <button type="submit">Submit</button>
        </form>
    </div>
</template>
<script>
import axios from 'axios';
import Helper from '../helper';

export default {
    mixins: [Helper],
    name: 'Login',
    created() {
        if (this.isLoggedIn()) {
            this.$router.push("dashboard");
        }
    },
    data() {
        return {
            user: 
            {
                username: '',
                password: ''
            },
            error: ''
        }
    },

    methods: {
        handleSubmit() {
            this.error = '';
            axios.post("/api/login", this.user).then((res) => {
                localStorage.setItem("token", res.data.token);
                this.$router.push("dashboard");
            }).catch((err)=> {
                if (err.response.status == 400) {
                    this.error = err.response.data.message;
                } else {
                    this.error = "Please try after some time";
                }

            });
        }
    }
}
</script>

<style>
label {
    display: block;
    font-size: 9pt;
    margin-top:0.5em;
    font-weight: bold;
}
input {
    display: block;
    margin-top: 0.5em;
    border: none;
    background-color: inherit;
    border-bottom: 1px solid lightslategrey;
    
}
input:focus{
    outline: none;
    border-bottom: 1px solid turquoise;
}
button {
    margin-top: 1em;
    background-color: teal;
    padding: 0.7em;
    color: wheat;
    border: none;
}

</style>