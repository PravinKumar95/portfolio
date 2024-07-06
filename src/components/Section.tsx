import React from 'react'
type SectionProps = {children: React.ReactNode}

const Section = ({children}:SectionProps) => {
  return (
    <section className='text-red-50'>{children}</section>
  )
}

export default Section