import Axios from "axios";

export default {
    methods: {
        isLoggedIn: function () {
            return localStorage.getItem("token") !== null;
        },

        getToken: function() {
            return localStorage.getItem("token");
        },

        logout: function() {
            Axios.delete("/api/logout").then(() => {
                this.$router.push("home");
            }).catch(() => {
                this.$router.push("home");
            });
            localStorage.clear();
      }
    }
  }