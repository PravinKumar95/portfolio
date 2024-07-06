import React from 'react'
import Navigation from './Navigation'
import Logo from './Logo'

const Header = () => {
  return (
    <header className='flex justify-between items-center p-8'>
        <Logo text='Pravin Kumar'/>
        <Navigation/>
    </header>
  )
}

export default Header