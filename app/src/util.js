import { useEffect, useRef } from 'react'
import { BrowserProvider, parseUnits, getAddress } from 'ethers'
import { BigNumber } from '@ethersproject/bignumber'
import { unionBy, uniqBy, keyBy, values as lodashvalues, merge } from 'lodash'

export async function getSigner() {
  await window.ethereum.request({ method: 'eth_requestAccounts' })
  return new BrowserProvider(window.ethereum).getSigner()
}

export function shorten(account) {
  if (!account) return ''
  return account.length > 15
    ? `${account.slice(0, 7)}...${account.slice(-5)}`
    : account
}

export function titleCase(s) {
  if (s) {
    if (s.length > 1) {
      return `${s[0].toUpperCase()}${s.slice(1)}`
    } else {
      return s.toUpperCase()
    }
  } else {
    return ''
  }
}

export function fmtDate(date) {
  return new Date(date)
    .toLocaleDateString('en-US', {
      dateStyle: 'long'
    })
    .replace(',', '')
}

export function dedupRegistryContacts(contacts) {
  const _contacts = contacts.map(a => ({ ...a }))
  const uniqs = uniqBy(_contacts, 'shieldedAddress')
  const merged = merge(
    keyBy(uniqs, 'shieldedAddress'),
    keyBy(_contacts, 'shieldedAddress')
  )
  const deduped = lodashvalues(merged)
  return deduped
}

export function copyToClipboard(text, e) {
  if (navigator.clipboard) {
    navigator.clipboard
      .writeText(text)
      .then(() => {
        if (e.target) {
          e.target.title = '✔️ Copied'
          setTimeout(() => {
            e.target.title = ''
          }, 4190)
        }
      })
      .catch(() => {})
  }
}
//FIXME needs an extra parameter $decimals to account for stables and exotics
export function prettierBalance(amount) {
  let hadDot = false
  let tokenAmount = amount
    ?.toString()
    .replace(/[^0-9\.]+/g, '')
    .slice(0, 18)
    .split('')
    .map(c => {
      if (c === '.' && hadDot) {
        return ''
      } else if (c === '.' && !hadDot) {
        hadDot = true
        return c
      } else {
        return c
      }
    })
    .join('')

  if (tokenAmount?.match(/(?<=\.)\d+/)?.[0].length > 4) {
    tokenAmount = Number(tokenAmount).toFixed(4)
  }

  return tokenAmount
}

export function setIntervalX(fn, ms, x) {
  let count = 0
  let id = setInterval(() => {
    fn()
    if (++count === x) clearInterval(id)
  }, ms)
  return id
}

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms))
}

export function gelatoTaskUrl(taskId) {
  return `https://relay.gelato.digital/tasks/status/${taskId}`
}

function pollGelatoTask(taskId) {
  return fetch(gelatoTaskUrl(taskId))
    .then(res => res.json())
    .then(res => {
      if (
        res.task.taskState === 'ExecReverted' ||
        res.task.taskState === 'Blacklisted' ||
        res.task.taskState === 'Cancelled' ||
        res.task.taskState === 'NotFound'
      ) {
        throw Error(`Gelato task ${taskId} failed`)
      }
      if (res.task.taskState === 'ExecSuccess') {
        return res.task.transactionHash
      }
    })
}

export async function fetchGelatoRelayTx(taskId, timeout = 1000, tries = 90) {
  let tx = await pollGelatoTask(taskId)
  while (!tx && --tries) {
    await sleep(timeout)
    tx = await pollGelatoTask(taskId)
  }
  return tx
}
///FIXME unhardcode decimals
export function gte(a = '0x00', b = '0.00', decimals = 18) {
  a = BigNumber.from(a)
  b = BigNumber.from(parseUnits(b, decimals))
  return a.gte(b)
}

export async function circlesSafesOf(owner) {
  const safeAddresses = await fetch(
    'https://api.thegraph.com/subgraphs/name/circlesubi/circles-ubi',
    {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body:
        '{"query":"{\\n  user(id: \\"' +
        owner.toLowerCase() +
        '\\") {\\n    safeAddresses\\n  }\\n}","variables":null,"extensions":{"headers":null}}'
    }
  )
    .then(r => (r.status !== 200 ? undefined : r.json()))
    .then(r => r?.data?.user?.safeAddresses || [])

  const safeAddressCopy = JSON.parse(JSON.stringify(safeAddresses))
  const batches = []
  while (safeAddressCopy.length) {
    batches.push(safeAddressCopy.splice(0, 50))
  }
  const circlesSafeMap = {}
  if (batches.length === 0) {
    return circlesSafeMap
  }
  const safes = []
  for (let batch of batches) {
    const query = batch.reduce((p, c) => p + `address[]=${getAddress(c)}&`, '')
    console.log('query', query)
    const requestUrl = `https://api.circles.garden/api/users/?${query}`
    const requestResultJson = await fetch(requestUrl).then(r => r.json())
    console.log('requestResultJson', requestResultJson)
    const profiles =
      requestResultJson.data.map(o => {
        return {
          // type: "Person",
          userName: o.username,
          userAvatar: o.avatarUrl,
          safeAddress: o.safeAddress.toLowerCase()
          // ownerAddress: ownerAddress.toLowerCase(),
        }
      }) ?? []

    Array.prototype.push.apply(safes, profiles)
  }
  return safes
}

// creating the custom useInterval hook
export function useInterval(callback, delay) {
  // Creating a ref
  const savedCallback = useRef()

  // To remember the latest callback .
  useEffect(() => {
    savedCallback.current = callback
  }, [callback])

  // combining the setInterval and
  //clearInterval methods based on delay.
  useEffect(() => {
    function func() {
      savedCallback.current()
    }
    if (delay !== null) {
      let id = setInterval(func, delay)
      return () => clearInterval(id)
    }
  }, [delay])
}