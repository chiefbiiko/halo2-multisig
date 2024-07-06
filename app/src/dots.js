import { useSelector } from "react-redux"
import { Box, Flex } from "rebass"

export default function Dots() {
  const dots = useSelector(state => state.dots)
  return dots ? (
    <Flex
      sx={{
        justifyContent: "center",
        alignItems: "center",
        height: "100vh",
        background: "rgba(255,255,255,.75)",
        position: "fixed",
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
        zIndex: 419
      }}
    >
      <Box className="three-dots-pulse" sx={{ background: "transparent" }} />
    </Flex>
  ) : null
}