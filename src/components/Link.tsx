import React from 'react'

type LinkProps = {
    text: string,
    url:  string
  }
const Link = ({text, url}: LinkProps) => {
  return (
    <a className='hover:text-blue-50' href={url}>{text}</a>
  )
}

export default Link