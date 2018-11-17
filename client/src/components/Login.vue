<template>
    <div class="Login">
        <form @submit.prevent="handleSubmit">
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

export default {
    name: 'Login',
    data() {
        return {
            user: 
            {
                name: '',
                password: ''
            }
        }
    },

    methods: {
        handleSubmit() {
            axios.post("/api/login", this.user).then((res) => {
                console.log(res);
            }).catch((err)=> {
                console.log(err);
            })
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