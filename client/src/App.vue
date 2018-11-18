<template>
  <div id="app">
    <nav>
      <router-link to="/">Home</router-link>
      <router-link to="/login" v-if="!isLoggedIn()">Login</router-link>
      <a href="#" v-on:click="logout()" v-if="isLoggedIn()">Logout</a>
    </nav>
    <router-view/>
  </div>
</template>

<script>
import Helper from './helper.js';

export default {
  mixins: [Helper],
  name: 'app',
}
</script>

<style>
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
}

body {
  background-color: #EEEEEE;
  font-family: 'Montserrat', sans-serif;
  display: grid;
  grid-template-rows: auto;
  margin: 50px;
}

nav {
  padding: 20px 20px 20px 0;
}

nav a {
  padding: 10px;
  text-decoration: none;
  background: #fff;
  border-radius: 3px;
  color: rgb(0, 110, 255);
  font-weight: bold;
  margin-right: 15px;
}

.error {
    color: red;
    font-size: 10pt;
}
</style>
