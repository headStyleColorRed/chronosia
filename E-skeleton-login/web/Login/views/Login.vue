<template>
    <div class="login">
        <!--- - - L E F T   C O N T A I N E R - - - -->
        <div class="leftContainer">
            <!--- - - left container top - - - -->
            <div class="leftContainer-top">
                <p v-if="isShowingLogin">
                    {{ Strings.notAMemberYet }}
                    <b class="header_login" v-on:click="changeLoginScreen()">{{
                        Strings.signUp
                    }}</b>
                </p>
                <p v-else>
                    {{ Strings.alreadyAMember }}
                    <b class="header_login" v-on:click="changeLoginScreen()">{{
                        Strings.signIn
                    }}</b>
                </p>
            </div>

            <!--- - - left container medium - - - -->
            <div class="leftContainer-medium">
                <h2 class="leftContainer-medium-title">
                    {{ isShowingLogin ? Strings.signIn : Strings.signUp }}
                </h2>
                <login-form
                    :loginStatus="isShowingLogin"
                    v-on:launchLoginProcess="processUserInputData"
                />
            </div>
        </div>

        <!--- - - R I G H T   C O N T A I N E R  - - - -->
        <div class="rightContainer">
            <p class="inspirationalQuote">
                CREATE A
                <span class="inspirationalQuoteEnding">PROJECT</span>
                <br />FIND A
                <span class="inspirationalQuoteEnding">TEAM</span>
                <br />LAUNCH YOUR
                <span class="inspirationalQuoteEnding">STARTUP</span>
            </p>
        </div>

        <!--- - - L O A D E R  - - - -->
        <div v-if="loading" class="progressWrapper">
            <v-progress-circular
                indeterminate
                color="primary"
                class="loader"
            ></v-progress-circular>
        </div>
    </div>
</template>

<script>
import stringsImport from "../assets/Strings_en.json";
import loginForm from "./loginForm.vue";
import validation from "../utils/validation.js";
import login_services from "../services/loginServices.js";
export default {
    name: "login",
    data() {
        return {
            loading: false,
            isShowingLogin: true,
            Strings: new Object(),
            mobileStyle: "",
        };
    },
    mounted() {
        this.Strings = stringsImport.screen_login;
    },
    methods: {
        changeLoginScreen() {
            this.isShowingLogin = !this.isShowingLogin;
        },
        processUserInputData(userData) {
            if (this.isShowingLogin) this.parseLoginData(userData);
            else this.parseRegisteringData(userData);
        },
        async parseLoginData(userData) {
            console.log("Login -> Validating");
            // Validation
            let validationResult = validation.validateRegisterData(userData);
            if (validationResult.isError) {
                return alert(validationResult.errorMessage);
            }
            console.log("Login -> Validation ok");

            // Login
            console.log("Login -> Login user");
            this.loading = true
            let loginResponse = await login_services
                .logUser(userData.username, userData.password)
                .catch((err) => {
                    console.log(err);
                    this.loading = false
                    return alert(err);
                });
            if (loginResponse.code != "200") {
                this.loading = false
                return alert(loginResponse.status);
            }
            console.log("Login -> Login Success");

            // Retrieve User
            console.log("Login -> Retrieving user");
            let user = await user_services
                .findUser(userData.username)
                .catch((err) => {
                    this.loading = false
                    return alert(err);
                });
            let userExists = user.data.length > 0;
            console.log("Login -> User retrieved");

            // User doesn't exist
            if (!userExists) {
                console.log("Login -> User hasn't been created yet");

                this.$store.commit("setUserLoginData", {
                    organizationId: null,
                    authToken: loginResponse.token,
                    email: userData.username,
                });

                return this.$router.push("/create_user");
            }

            // User exists but has no organization
            let userHasOrganizations = user.data[0].organizations.length > 0;
            console.log(
                `Login -> User has ${
                    userHasOrganizations ? "" : "no"
                } organizations.`
            );
            if (!userHasOrganizations) {
                this.$store.commit("setUserLoginData", {
                    organizationId: null,
                    authToken: loginResponse.token,
                    email: userData.username,
                });

                return this.$router.push("/create_organization");
            }

            // User has single organization
            if (user.data[0].organizations.length == 1) {
                console.log(
                    "Login -> Moving to dashboard for single organization"
                );
                return this.logUser({
                    organizationId: user.data[0].organizations[0].id,
                    authToken: loginResponse.token,
                    email: userData.username,
                });
            }

            // User has multiple organizations
            if (user.data[0].organizations.length > 1) {
                console.log("Login -> Moving to choose organization");
                this.logUser({
                    organizationId: null,
                    authToken: loginResponse.token,
                    email: userData.username,
                });
                return this.$router.push("/create_organization");
            }
        },
        parseRegisteringData(userData) {
            // Validation
            let validationResult = validation.validateRegisterData(userData);
            if (validationResult.isError) {
                return alert(validationResult.errorMessage);
            }

            // Register User
            console.log("Login ->  rEgistering user");
            this.loading = true
            login_services
                .registerUser(
                    userData.username,
                    userData.password,
                    userData.passwordConfirmation
                )
                .then((res) => {
                    if (res.code != "200") {
                        this.loading = false
                        return alert(res.status);
                    }
                    this.parseLoginData(userData);
                });
        },
        logUser(userData) {
            // Save login data
            this.$store.commit("setUserLoginData", userData);

            // Handle after login flow path
            if (userData.organizationId == null) {
                this.$router.push("/create_user");
            } else {
                this.$router.push("/");
            }
        },
    },
    watch: {
        isShowingLogin(val) {
            if (val == false) this.mobileStyle = "display: none;";
            else this.mobileStyle = "";
        },
    },
    components: {
        "login-form": loginForm,
    },
};
</script>

<style scoped>
.progressWrapper {
    background-color: rgba(27, 100, 134, 0);
    width: 100%;
    height: 100%;
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    display: grid;
}

.loader {
    justify-self: center;
    align-self: center;
}

/*-------------- IPHONE 5 ----------------*/
@media only screen and (min-device-width: 320px) and (max-device-width: 568px) and (orientation: portrait) {
    .login {
        height: 100vh;
        width: 100vw;
        display: grid;
        grid-template-columns: 1.25fr 0;
        color: #666666;
    }
    /* - - - - - - - - L E F T   C O N T A I N E R - - - - - */
    .leftContainer {
        position: relative;
        background-color: rgb(255, 255, 255);
        height: 100vh;
        display: grid;
        grid-template-rows: 0.5fr 4fr 1fr;
    }
    /* - - - left container top - - - */
    .leftContainer-top p {
        padding: 2rem;
        padding-bottom: 0rem;
        text-align: right;
    }
    .header_login {
        font-weight: 400;
        color: rgb(61, 110, 5);
        cursor: pointer;
    }
    /* - - - left container medium - - - */
    .leftContainer-medium {
        position: relative;
        justify-self: center;
    }
    .leftContainer-medium-title {
        font-size: 2.5rem;
        font-weight: 300;
        margin-bottom: 2rem;
    }
    /* - - - left container bottom - - - */
    .leftContainer-bottom-web {
        display: none;
    }
    .leftContainer-bottom-mobile {
        display: grid;
    }
    .divisorsContainer-mobile {
        display: grid;
        grid-template-columns: 3fr 1fr 3fr;
    }
    .divisors-mobile {
        border: 0.1px solid rgb(223, 223, 223);
    }
    #divisorText-mobile {
        text-align: center;
        margin-top: -0.6rem;
        margin-left: -0.2rem;
        color: rgb(179, 179, 179);
        font-size: 0.9rem;
    }
    .externalLoginButtonsMobile {
        display: grid;
        grid-template-columns: 1fr 1fr;
        padding: 0 3rem;
    }
    .externalLogin-mobile {
        justify-self: center;
    }
    .externalLogin-mobile img {
        height: 2rem;
    }
    /* - - - - - - - R I G H T   C O N T A I N E R - - - - - */
    .rightContainer {
        display: none;
    }
}
/*-------------- IPHONE 6 ----------------*/
@media only screen and (min-device-width: 375px) and (max-device-width: 667px) and (orientation: portrait) {
    .leftContainer {
        grid-template-rows: 1fr 4fr 1fr;
    }
}
/*---------------- IPAD ----------------*/
@media only screen and (min-device-width: 768px) and (max-device-width: 1024px) {
    .login {
        height: 100vh;
        width: 100vw;
        display: grid;
        grid-template-columns: 1.25fr 0;
        color: #666666;
    }
    /* - - - - - - - - L E F T   C O N T A I N E R - - - - - */
    .leftContainer {
        position: relative;
        background-color: rgb(255, 255, 255);
        height: 100vh;
        display: grid;
        grid-template-rows: 1.5fr 4fr 1fr;
    }
    /* - - - left container top - - - */
    .leftContainer-top p {
        padding: 2rem;
        padding-bottom: 0rem;
        text-align: right;
    }
    .header_login {
        font-weight: 400;
        color: rgb(61, 110, 5);
        cursor: pointer;
    }
    /* - - - left container medium - - - */
    .leftContainer-medium {
        position: relative;
        justify-self: center;
    }
    .leftContainer-medium-title {
        font-size: 2.5rem;
        font-weight: 300;
        margin-bottom: 2rem;
    }
    /* - - - left container bottom - - - */
    .leftContainer-bottom-web {
        display: none;
    }
    .leftContainer-bottom-mobile {
        display: grid;
    }
    .divisorsContainer-mobile {
        display: grid;
        grid-template-columns: 3fr 1fr 3fr;
    }
    .divisors-mobile {
        border: 0.1px solid rgb(223, 223, 223);
    }
    #divisorText-mobile {
        text-align: center;
        margin-top: -0.6rem;
        margin-left: -0.2rem;
        color: rgb(179, 179, 179);
        font-size: 0.9rem;
    }
    .externalLoginButtonsMobile {
        display: grid;
        grid-template-columns: 1fr 1fr;
        padding: 0 3rem;
    }
    .externalLogin-mobile {
        justify-self: center;
    }
    .externalLogin-mobile img {
        height: 2rem;
    }
    /* - - - - - - - R I G H T   C O N T A I N E R - - - - - */
    .rightContainer {
        display: none;
    }
}
/*-------------- HALF SCREEN ----------------*/
@media only screen and (min-device-width: 500px) and (max-width: 1000px) {
    .login {
        height: 100vh;
        width: 100vw;
        display: grid;
        grid-template-columns: 1.25fr 0;
        color: #666666;
    }
    /* - - - - - - - - L E F T   C O N T A I N E R - - - - - */
    .leftContainer {
        position: relative;
        background-color: rgb(255, 255, 255);
        height: 100vh;
        display: grid;
        grid-template-rows: 1.5fr 4fr 1fr;
    }
    /* - - - left container top - - - */
    .leftContainer-top p {
        padding: 2rem;
        padding-bottom: 0rem;
        text-align: right;
    }
    .header_login {
        font-weight: 400;
        color: rgb(61, 110, 5);
        cursor: pointer;
    }
    /* - - - left container medium - - - */
    .leftContainer-medium {
        position: relative;
        justify-self: center;
    }
    .leftContainer-medium-title {
        font-size: 2.5rem;
        font-weight: 300;
        margin-bottom: 2rem;
    }
    /* - - - left container bottom - - - */
    .leftContainer-bottom-web {
        display: none;
    }
    .leftContainer-bottom-mobile {
        display: grid;
    }
    .divisorsContainer-mobile {
        display: grid;
        grid-template-columns: 3fr 1fr 3fr;
    }
    .divisors-mobile {
        border: 0.1px solid rgb(223, 223, 223);
    }
    #divisorText-mobile {
        text-align: center;
        margin-top: -0.6rem;
        margin-left: -0.2rem;
        color: rgb(179, 179, 179);
        font-size: 0.9rem;
    }
    .externalLoginButtonsMobile {
        display: grid;
        grid-template-columns: 1fr 1fr;
        padding: 0 3rem;
    }
    .externalLogin-mobile {
        justify-self: center;
    }
    .externalLogin-mobile img {
        height: 2rem;
    }
    /* - - - - - - - R I G H T   C O N T A I N E R - - - - - */
    .rightContainer {
        display: none;
    }
}
/*-------------- FULL SCREEN ----------------*/
@media only screen and (min-width: 1001px) and (max-width: 1450px) {
    .login {
        height: 100vh;
        width: 100vw;
        display: grid;
        grid-template-columns: 1.25fr 1fr;
        color: #666666;
    }
    /* - - - - - - - - L E F T   C O N T A I N E R - - - - - */
    .leftContainer {
        position: relative;
        background-color: rgb(255, 255, 255);
        height: 100vh;
        display: grid;
        grid-template-rows: 1fr 4fr 1fr;
    }
    /* - - - left container top - - - */
    .leftContainer-top p {
        padding: 2rem;
        text-align: right;
    }
    .header_login {
        font-weight: 400;
        color: rgb(61, 110, 5);
        cursor: pointer;
    }
    /* - - - left container medium - - - */
    .leftContainer-medium {
        position: relative;
        padding: 0 10rem;
    }
    .leftContainer-medium-title {
        font-size: 2.5rem;
        font-weight: 300;
        margin-bottom: 2rem;
    }
    /* - - - left container bottom - - - */
    .leftContainer-bottom-mobile {
        display: none;
    }
    .leftContainer-bottom-web {
        padding: 0 10rem;
    }
    .divisorsContainer-web {
        display: grid;
        grid-template-columns: 3fr 1fr 3fr;
    }
    .divisors-web {
        border: 0.1px solid rgb(223, 223, 223);
    }
    #divisorText-web {
        text-align: center;
        margin-top: -0.6rem;
        margin-left: -0.2rem;
        color: rgb(179, 179, 179);
        font-size: 0.9rem;
    }
    .externalLoginButtons-web {
        display: grid;
        grid-template-columns: 1fr 1fr;
        grid-gap: 2rem;
    }
    .externalLogin-web {
        display: grid;
        border-radius: 3px;
        grid-template-columns: 1fr 5fr;
        border: 1px solid rgb(175, 175, 175);
        padding: 0.7rem;
        cursor: pointer;
    }
    .externalLogin-web:hover {
        border: none;
        background-color: rgb(221, 221, 221);
    }
    .externalLogin-web img {
        height: 1.3rem;
        justify-self: center;
        align-self: center;
    }
    .externalLogin-web span {
        justify-self: center;
        align-self: center;
    }
    /* - - - - - - - R I G H T   C O N T A I N E R - - - - - */
    .rightContainer {
        background-color: rgb(238, 238, 238);
        height: 100vh;
        position: relative;
    }
    .inspirationalQuote {
        position: absolute;
        top: 20vh;
        left: 2vw;
        font-size: 1.5rem;
        font-weight: 300;
        margin-bottom: 2rem;
    }
    .inspirationalQuoteEnding {
        color: rgb(61, 110, 5);
        font-weight: 400;
    }
}
/*-------------- MONITOR SCREEN ----------------*/
@media only screen and (min-width: 1450px) {
    .login {
        height: 100vh;
        width: 100vw;
        display: grid;
        grid-template-columns: 1.25fr 1fr;
        color: #666666;
    }
    /* - - - - - - - - L E F T   C O N T A I N E R - - - - - */
    .leftContainer {
        position: relative;
        background-color: rgb(255, 255, 255);
        height: 100vh;
        display: grid;
        grid-template-rows: 1fr 4fr 1fr;
    }
    /* - - - left container top - - - */
    .leftContainer-top p {
        padding: 2rem;
        text-align: right;
    }
    .header_login {
        font-weight: 400;
        color: rgb(61, 110, 5);
        cursor: pointer;
    }
    /* - - - left container medium - - - */
    .leftContainer-medium {
        justify-self: center;
    }
    .leftContainer-medium-title {
        font-size: 2.5rem;
        font-weight: 300;
        margin-bottom: 2rem;
    }
    /* - - - left container bottom - - - */
    .leftContainer-bottom-mobile {
        display: none;
    }
    .leftContainer-bottom-web {
        padding: 0 10rem;
    }
    .divisorsContainer-web {
        display: grid;
        grid-template-columns: 3fr 1fr 3fr;
    }
    .divisors-web {
        border: 0.1px solid rgb(223, 223, 223);
    }
    #divisorText-web {
        text-align: center;
        margin-top: -0.6rem;
        margin-left: -0.2rem;
        color: rgb(179, 179, 179);
        font-size: 0.9rem;
    }
    .externalLoginButtons-web {
        display: grid;
        grid-template-columns: 1fr 1fr;
        grid-gap: 2rem;
    }
    .externalLogin-web {
        display: grid;
        border-radius: 3px;
        grid-template-columns: 1fr 5fr;
        border: 1px solid rgb(175, 175, 175);
        padding: 0.7rem 2rem;
        cursor: pointer;
        justify-self: center;
    }
    .externalLogin-web:hover {
        border: none;
        background-color: rgb(221, 221, 221);
    }
    .externalLogin-web img {
        height: 1.3rem;
        justify-self: center;
        align-self: center;
    }
    .externalLogin-web span {
        justify-self: center;
        align-self: center;
    }
    /* - - - - - - - R I G H T   C O N T A I N E R - - - - - */
    .rightContainer {
        background-color: rgb(238, 238, 238);
        height: 100vh;
        position: relative;
    }
    .inspirationalQuote {
        position: absolute;
        top: 15vh;
        left: 4vw;
        font-size: 1.5rem;
        font-weight: 300;
        margin-bottom: 2rem;
    }
    .inspirationalQuoteEnding {
        color: rgb(61, 110, 5);
        font-weight: 400;
    }
}
</style>