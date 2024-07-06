import React from 'react'
import { Flex, Box } from 'rebass'
// import { Input, Checkbox, Label, Textarea } from '@rebass/forms'
import { Input } from '@rebass/forms'
import { useDispatch, useSelector } from 'react-redux'
import Button from './button'
// import StandardTokenInput from './std-token-input'
import { dump, sign  } from './redux'
// import { gte } from './util'

// function validate(shieldedRecipient, amount, balance) {
//   return (
//     !!shieldedRecipient &&
//     /^\d+(\.\d+)?$/.test(amount) &&
//     gte(balance, amount) &&
//     Number(amount) !== 0
//   )
// }

export default function Sign() {
//   const { account } = useMyMetaMask()
  const {
    // fundShieldedRecipient,
    // tokenAmount,
    // transferNote,
    // shieldedAddress,
    // selectedToken,
    // standardBalance,
    // frens = [],
    // name
    masterSafe,
    oldSigner,
    newSigner
  } = useSelector(state => state)
  const dispatch = useDispatch()
//   const valid = validate(
//     fundShieldedRecipient || shieldedAddress,
//     tokenAmount,
//     standardBalance[selectedToken]
//   )
const valid = masterSafe && oldSigner && newSigner
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
        Sign
      </Flex>

      <Input
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
      />


<Input
        type="text"
        id="oldSigner"
        placeholder="Old signer"
        title="0x..."
        onChange={e =>
          dispatch(dump({ oldSigner: e.target.value }))
        }
        value={oldSigner ?? ''}
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
          dispatch(dump({ newSigner: e.target.value }))
        }
        value={newSigner ?? ''}
        bg="#fff"
        sx={{
          marginBottom: '0.625em',
          fontSize: [16, 18, 20],
          textAlign: 'center'
        }}
      />

      {/* <datalist id="favorite-contacts">
        {frens?.map(f => {
          const val = f.name || f.shieldedAddress
          return (
            <option key={val} value={val}>
              {val}
            </option>
          )
        })}
      </datalist>
      <StandardTokenInput />

      <Textarea
        value={transferNote}
        rows={1}
        placeholder="Note (optional)"
        maxLength={255}
        sx={{
          textAlign: 'center',
          resize: 'none',
          fontSize: [12, 14],
          marginBottom: '0.625em'
        }}
        onChange={e => dispatch(dump({ transferNote: e.target.value }))}
      ></Textarea> */}

      <Button
        disabled={!valid}
        aria-label="Fund"
        style={{
          cursor: valid ? 'pointer' : 'not-allowed'
        }}
        onClick={() =>
          dispatch(sign(masterSafe, oldSigner, newSigner))
        }
      >
        Sign msg via Safe
      </Button>
    </Box>
  )
}