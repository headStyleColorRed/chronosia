export default {
    state: {
        email: null,
        authToken: null,
        isLogged: false,
    },
    mutations: {
        setState(state) {

        },
        resetAuthState(state) {
            state.authToken = null
            state.email = null;
            state.isLogged = false;
        },
    },
    getters: {
        isLogged: (state) => {
            return state.isLogged;
        },
        authToken: (state) => {
            return state.authToken;
        },
        email: (state) => {
            return state.email;
        },
    }
}