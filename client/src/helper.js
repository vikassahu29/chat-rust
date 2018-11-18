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
            let token = this.getToken();
            if (token !== null) {
                Axios.delete("/api/logout", {
                    headers: {
                        "authorization": "Bearer " + token
                    }
                }).then(() => {
                    this.$router.push("/");
                }).catch(() => {
                    this.$router.push("/");
                });
            }
            localStorage.clear();
        },
        
        getUsername: function() {
            return localStorage.getItem("username");
        }
    }
  }