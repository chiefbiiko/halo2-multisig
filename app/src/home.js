import React from "react"
import { Flex, Box, Text, Button } from "rebass"
import { Input, Checkbox, Label } from "@rebass/forms"
import { useDispatch, useSelector } from "react-redux"
import { dump, signMsg } from "./redux"

export default function HomeScreen() {
  const { masterSafe, oldSigner, newSigner } = useSelector(
    state => state
  )
  const dispatch = useDispatch()

  return (
    <Box
      sx={{
        margin: "10vh auto auto auto",
        background: "#fff",
        padding: "0 0.625em",
        maxWidth: "24em"
      }}
    >
      <Flex
        sx={{
          fontWeight: "bold",
          justifyContent: "center",
          marginBottom: "0.625em",
          flexWrap: "wrap"
        }}
      >
        <a
          href="/"
          style={{
            textDecoration: "none",
            fontSize: 36,
            fontWeight: "bolder",
            color: "#000",
            verticalAlign: "top",
            alignSelf: "flex-end"
          }}
        >
          {process.env.REACT_APP_PROJECT_NAME}
        </a>
      </Flex>

      <Input
        type="text"
        id="masterSafe"
        placeholder="Master Safe"
        title="0x..."
        onChange={e => dispatch(dump({ masterSafe: e.target.value }))}
        value={masterSafe || ""}
        bg="#fff"
        sx={{ marginBottom: "0.625em" }}
      />

      <Input
        type="text"
        id="oldSigner"
        placeholder="Old signer"
        title="0x..."
        onChange={e => dispatch(dump({ oldSigner: e.target.value }))}
        value={oldSigner || ""}
        bg="#fff"
        sx={{ marginBottom: "0.625em" }}
      />

      <Input
        type="text"
        id="newSigner"
        placeholder="New signer"
        title="0x..."
        onChange={e => dispatch(dump({ newSigner: e.target.value }))}
        value={newSigner || ""}
        bg="#fff"
        sx={{ marginBottom: "0.625em" }}
      />

      <Button
        disabled={!(masterSafe && oldSigner && newSigner)}
        aria-label="sign msg"
        type="submit"
        sx={{
          width: "100%",
          marginTop: "0.625em",
          marginBottom: "1.25em",
          bg: "#000",
          cursor:
            masterSafe && oldSigner && newSigner
              ? "pointer"
              : "none",
          fontWeight: "bold"
        }}
        onClick={() => dispatch(signMsg(masterSafe, oldSigner, newSigner))}
      >
        Sign msg via Safe
      </Button>

      <hr/>
        {/* TODO TBC */}





      {/* <Button
        disabled={
          !SUBSTRATE_ADDRESS_PATTERN.test(substrateAddress) || !termsAccepted
        }
        aria-label="request test coins"
        type="submit"
        sx={{
          width: "100%",
          marginTop: "0.625em",
          marginBottom: "1.25em",
          bg: "#000",
          cursor:
            SUBSTRATE_ADDRESS_PATTERN.test(substrateAddress) && termsAccepted
              ? "pointer"
              : "none",
          fontWeight: "bold"
        }}
        onClick={() => dispatch(fund(substrateAddress))}
      >
        GET {process.env.REACT_APP_AMOUNT}T0RN
      </Button> */}
    </Box>
  )
}