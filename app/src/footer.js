import React from "react"
import { Flex, Text, Box, Link } from "rebass"

export default function Footer() {
  return (
    <Box bg="#fff">
      <Flex
        sx={{
          color: "#000",
          padding: "0.625em",
          margin: "auto 0 0 0",
          fontSize: [16, 18, 20]
        }}
      >
        <Text>
          {"🏈 " +
            new Date().getFullYear() +
            " " +
            process.env.REACT_APP_AUTHOR_NAME}
        </Text>
        <Box mx="auto" />
        <Link
          href={process.env.REACT_APP_PDOT_APPS_URL}
          target="_blank"
          style={{
            color: "#000",
            margin: "0 0 0 0.625em",
            textDecoration: "none",
            fontWeight: "bolder",
            cursor: "pointer"
          }}
        >
          <span>_</span>
        </Link>
      </Flex>
    </Box>
  )
}