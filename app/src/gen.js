import React from 'react'
import { Flex, Box } from 'rebass'
// import { Input, Checkbox, Label, Textarea } from '@rebass/forms'
import { Input } from '@rebass/forms'
import { useDispatch, useSelector } from 'react-redux'
import Button from './button'
// import StandardTokenInput from './std-token-input'
import { dump, gen  } from './redux'

export default function Gen() {
//   const { account } = useMyMetaMask()
  const {
    masterSafe,
    // oldSigner,
    // newSigner
    msgHash
  } = useSelector(state => state)
  const dispatch = useDispatch()

const valid = masterSafe && msgHash
  return (
    <Box
      sx={{
        maxWidth: '24em',
        border: '0.1875em solid #000',
        boxShadow: '0 0.625em',
        padding: '0.625em'
      }}
    >
      <Flex
        sx={{
          fontSize: [24, 26, 28],
          fontWeight: 'bold',
          marginBottom: '0.625em',
          flexWrap: 'wrap'
        }}
      >
        🅰️🅰️🅰️®
      </Flex>

      {/* <Input
        type="text"
        id="masterSafe"
        placeholder="Master Safe"
        title="0x..."
        onChange={e =>
          dispatch(dump({ masterSafe: e.target.value }))
        }
        value={masterSafe ?? ''}
        bg="#fff"
        sx={{
          marginBottom: '0.625em',
          fontSize: [16, 18, 20],
          textAlign: 'center'
        }}
      /> */}


{/* <Input
        type="text"
        id="subSafe"
        placeholder="Sub Safe"
        title="0x..."
        onChange={e =>
          dispatch(dump({ msgHash: e.target.value }))
        }
        value={msgHash ?? ''}
        bg="#fff"
        sx={{
          marginBottom: '0.625em',
          fontSize: [16, 18, 20],
          textAlign: 'center'
        }}
      /> */}

<Input
        type="text"
        id="oldSigner"
        placeholder="Old signer"
        title="0x..."
        onChange={e =>
          dispatch(dump({ msgHash: e.target.value }))
        }
        value={msgHash ?? ''}
        bg="#fff"
        sx={{
          marginBottom: '0.625em',
          fontSize: [16, 18, 20],
          textAlign: 'center'
        }}
      />

<Input
        type="text"
        id="newSigner"
        placeholder="New signer"
        title="0x..."
        onChange={e =>
          dispatch(dump({ msgHash: e.target.value }))
        }
        value={msgHash ?? ''}
        bg="#fff"
        sx={{
          marginBottom: '0.625em',
          fontSize: [16, 18, 20],
          textAlign: 'center'
        }}
      />

      <Button
        disabled={!valid}
        aria-label="Gen"
        style={{
          cursor: valid ? 'pointer' : 'not-allowed'
        }}
        onClick={() =>
          dispatch(gen(masterSafe, msgHash))
        }
      >
        Recover
      </Button>
    </Box>
  )
}