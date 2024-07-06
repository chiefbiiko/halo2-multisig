import { configureStore } from "@reduxjs/toolkit"

// const { default: Safe, EthersAdapter } = require("@safe-global/protocol-kit")
// const { default: SafeApiKit } = require("@safe-global/api-kit")
const ethers = require("ethers")

// import { Safe, EthersAdapter } from "@safe-global/protocol-kit"
// import SafeApiKit from "@safe-global/api-kit"
// import ethers from "ethers"

const DUMP = "DUMP"

export function dump(props) {
  return { type: DUMP, ...props }
}

export function signMsg(masterSafe, oldSigner, newSigner) {
    return async function (dispatch, getState) {
        dispatch(dump({ dots: true }))
        //TODO



        dispatch(dump({ dots: false }))
    }
}

export const store = configureStore({
  reducer(state = {}, { type, ...props }) {
    switch (type) {
      case DUMP:
        return { ...state, ...props }
      default:
        return state
    }
  }
})