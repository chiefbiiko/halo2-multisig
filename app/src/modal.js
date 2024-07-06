import { default as ReactModal } from "react-modal"
import { useSelector, useDispatch } from "react-redux"
import { Text, Button, Flex, Box } from "rebass"
import { dump } from "./redux"

export default function Modal() {
  const dispatch = useDispatch()
  const { modalText, modalTitle } = useSelector(state => state)
  return (
    <ReactModal
      isOpen={!!modalText}
      onRequestClose={() => dispatch(dump({ modalText: null }))}
      contentLabel="info overlay modal"
      appElement={document.getElementById("root")}
    >
      <Flex
        style={{
          justifyContent: "center",
          marginTop: window.innerHeight < 300 ? "0" : "20vh"
        }}
      >
        <Box
          sx={{
            background: "#fff",
            border: "0.1875em solid #000",
            boxShadow: "0 0.625em",
            padding: "0.625em",
            overflow: "hidden",
            textAlign: "center",
            fontSize: ["1em", "1.25em"]
          }}
        >
          <Text sx={{ fontWeight: "bold", marginBottom: "0.625em" }}>
            {modalTitle}
          </Text>
          <Text sx={{ font: "1em Open Sans, normal", marginBottom: "0.625em" }}>
            {modalText}
          </Text>
          <Button
            style={{
              color: "#000",
              cursor: "pointer",
              margin: "1.25em 0.625em 0 0.625em",
              fontWeight: "bold"
            }}
            onClick={() => dispatch(dump({ modalText: null }))}
          >
            ok
          </Button>
          <div style={{ clear: "both" }}></div>
        </Box>
      </Flex>
    </ReactModal>
  )
}