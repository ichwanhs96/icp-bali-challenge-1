import { useState } from 'react';
import { icp_workshop_backend } from 'declarations/icp-workshop-backend';

function App() {
  const [greeting, setGreeting] = useState('');
  const [stockPrice, setStockPrice] = useState('');

  function handleSubmit(event) {
    event.preventDefault();
    const name = event.target.elements.name.value;
    icp_workshop_backend.get_stock_price(name).then((response) => {
      setStockPrice(JSON.parse(response));
    });
    return false;
  }

  return (
    <main>
      <div style={{ backgroundColor: '#2e2e2e', color: '#d3c6b0', fontFamily: 'Arial, sans-serif', padding: '20px', height: '100vh' }}>
        <img src="/logo2.svg" alt="DFINITY logo" style={{ filter: 'drop-shadow(0 0 10px #d3c6b0)' }} />
        <br/>
        <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
          <h1 style={{ textShadow: '0 0 5px #d3c6b0', textAlign: 'center' }}>💲 Simple Stock Price Retrieval</h1>
        </div>
        <br/>
        <form action="#" onSubmit={handleSubmit} style={{ border: '1px solid #d3c6b0', padding: '10px', backgroundColor: '#3a3a3a' }}>
          <label htmlFor="name" style={{ color: '#d3c6b0' }}>Enter stock symbol (e.g. "AAPL"): &nbsp;</label>
          <input id="name" alt="Name" type="text" style={{ backgroundColor: '#4a4a4a', color: '#d3c6b0', border: '1px solid #d3c6b0', padding: '12px' }} />
          <button type="submit" style={{ backgroundColor: '#d3c6b0', color: '#2e2e2e', border: 'none', padding: '5px 10px', cursor: 'pointer' }}>Retrieve stock price!</button>
        </form>
        <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
          <section id="stockPrice" style={{ color: '#d3c6b0', marginTop: '10px' }}>Retrieved stock price: {stockPrice.pc}</section>
        </div>
      </div>
    </main>
  );
}

export default App;
