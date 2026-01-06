<template>
  <div style="padding: 20px; font-family: sans-serif; max-width: 800px; margin: auto;">
    <h1>D&P Perfumum Gjilan</h1>

    <div style="background: #f9f9f9; padding: 20px; border-radius: 8px; margin-bottom: 30px; border: 1px solid #ddd;">
      <h3>Add New Stock Item</h3>
      <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 10px;">
        <input v-model="newItem.code" placeholder="Code (e.g Y5)"/>
        <input v-model="newItem.name" placeholder="Perfume Name (e.g. Libre)" />
        <input v-model="newItem.brand" placeholder="Brand (e.g. YSL)" />
        <input v-model.number="newItem.size" type="number" placeholder="Size (ml)" />
        <input v-model.number="newItem.price" type="number" placeholder="Price (€)" />
        <input v-model.number="newItem.stock" type="number" placeholder="Quantity in Stock" />
        <button @click="addPerfume" style="grid-column: span 2; background: #4CAF50; color: white; padding: 10px; border: none; cursor: pointer;">
          Save to Inventory
        </button>
      </div>
    </div>

    <table border="1" style="width: 100%; text-align: left; border-collapse: collapse;">
      <thead style="background-color: #f2f2f2;">
        <tr>
          <th style="padding: 10px;">Code</th>
          <th style="padding: 10px;">Name</th>
          <th style="padding: 10px;">Size</th>
          <th style="padding: 10px;">Price</th>
          <th style="padding: 10px;">Stock</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="perfume in perfumes" :key="perfume._id">
          <td style="padding: 10px;">{{ perfume.code }}</td>
          <td style="padding: 10px;"><strong>{{ perfume.brand }}</strong> {{ perfume.name }}</td>
          <td style="padding: 10px;">{{ perfume.size }}ml</td>
          <td style="padding: 10px;">€{{ perfume.price }}</td>
          <td style="padding: 10px;">{{ perfume.stock }} units</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import axios from 'axios';

// 1. Storage for our list and the new item we are typing
const perfumes = ref([]);
const newItem = ref({
  code: '',
  name: '',
  brand: '',
  size: '',
  price: '',
  stock: ''
});

// 2. Function to "GET" all perfumes from the database
const fetchPerfumes = async () => {
  try {
    const response = await axios.get('http://localhost:8000/api/perfumes');
    perfumes.value = response.data;
  } catch (error) {
    console.error("Error fetching data:", error);
  }
};

// 3. Function to "POST" a new perfume (including the ml size)
const addPerfume = async () => {
  try {
    // Send the newItem object to Rust
    await axios.post('http://localhost:8000/api/perfumes', newItem.value);
    
    // Refresh the list so the new item appears in the table
    await fetchPerfumes();
    
    // Clear the form so it's empty for the next entry
    newItem.value = { code: '',name: '', brand: '', size: '', price: '', stock: ''};
  } catch (error) {
    console.error("Error saving perfume:", error);
    alert("Check if your Rust backend is running!");
  }
};

onMounted(fetchPerfumes);
</script>