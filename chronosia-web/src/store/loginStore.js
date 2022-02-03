export default {
    state: {
        email: null,
        authToken: null,
        isLogged: false,
    },
    mutations: {
        setState(state, data) {
            state.email = data.email
            state.authToken = data.authToken
            state.isLogged = data.isLogged
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