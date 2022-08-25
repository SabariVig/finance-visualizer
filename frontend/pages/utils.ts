import axios from "axios"
import { BACKEND_URL } from "./consts"

const convertToLine = async (url: string, id: string, negate: number = 1) => {
  const { data: fetchedData } = await axios.get(url)
  const data = fetchedData.map(entry => ({ x: entry.date, y: entry.amount * negate }))
  return { id, data }
}

const convertToTreemap = async (url: string, name: string, negate: number = 1) => {
  const { data: fetchedData } = await axios.get(url)
  const children = fetchedData.map(entry => ({ name: entry.account, score: entry.amount * negate }))
  return { name, children }
}


const convertToBarChart = async (url: string, negate: number = 1) => {
  const { data: montlhyChart } = await axios.get(url)
  const data = montlhyChart.map(entry => ({ ...entry ,date: new Intl.DateTimeFormat('en-US', {month:"short",year:"2-digit"}).format(new Date(entry.date)) }))
  return data 
}

const numberFormatter = new Intl.NumberFormat('en-IN', {
  style: 'currency',
  currency: 'INR',
  minimumIntegerDigits: 2,
  currencyDisplay: 'symbol'
});

const formatNumberToINR = (num: number) => {
  return numberFormatter.format(num);
}


const averageDaily = async (account: string, negate: number = 1) => {
  const { data: cashflowExp } = await axios.get(`${BACKEND_URL}/cashflow/${account}?convert_commodity=true`)
  const totalExpenese = cashflowExp[cashflowExp.length - 1].amount * negate
  const startDate = cashflowExp[0].date
  const totalDays = Math.floor((new Date().getTime() - new Date(startDate).getTime()) / (1000 * 60 * 60 * 24))


  return {
    amount: totalExpenese / totalDays,
    account: `A:AverageDaily${account}`
  }
}


export { convertToLine, convertToTreemap, convertToBarChart, formatNumberToINR, averageDaily }
