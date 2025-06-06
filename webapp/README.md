# BleuScore Online - Web Application

A fast, modern web application for calculating BLEU scores using the `bleuscore-js` npm package. Perfect for evaluating machine translation quality directly in your browser.

## ✨ Features

- **Fast Calculation**: Powered by `bleuscore-js` for efficient BLEU score computation
- **Modern UI**: Beautiful, responsive interface built with Tailwind CSS
- **GitHub Pages Ready**: Static application that works perfectly on GitHub Pages
- **Real-time Results**: Instant BLEU score calculation with detailed metrics
- **N-gram Analysis**: Support for 1-gram to 5-gram precision analysis
- **Mobile Friendly**: Responsive design that works on all devices

## 🚀 Live Demo

Visit the live application: [BleuScore Online](https://your-username.github.io/bleuscore/webapp/)

## 📊 What is BLEU Score?

BLEU (Bilingual Evaluation Understudy) is a metric for automatically evaluating machine-translated text. It compares candidate translations with reference translations using n-gram precision and brevity penalty.

### Score Interpretation
- **0.8-1.0**: Excellent quality
- **0.6-0.8**: Good quality  
- **0.4-0.6**: Fair quality
- **0.0-0.4**: Poor quality

## 🛠️ Local Development

### Prerequisites
- Python 3.x (for local server)
- Modern web browser

### Running Locally

1. Clone the repository:
```bash
git clone https://github.com/shenxiangzhuang/bleuscore.git
cd bleuscore/webapp
```

2. Start a local server:
```bash
npm run serve
# or
python3 -m http.server 8080
```

3. Open your browser and navigate to `http://localhost:8080`

## 📦 Using bleuscore-js Package

This application uses the `bleuscore-js` npm package. You can also use it in your own projects:

### Installation
```bash
npm install bleuscore-js
```

### Usage
```javascript
import BleuScore from 'bleuscore-js';

const reference = "The cat is sitting on the mat.";
const prediction = "The cat is on the mat.";
const maxN = 4;

const result = BleuScore.compute(reference, prediction, maxN);
console.log('BLEU Score:', result.bleu);
console.log('Precisions:', result.precisions);
console.log('Brevity Penalty:', result.brevity_penalty);
```

## 🌐 GitHub Pages Deployment

This application is designed to work seamlessly with GitHub Pages:

1. **Fork the repository** or create your own
2. **Enable GitHub Pages** in repository settings
3. **Set source** to the `webapp` directory or copy files to root
4. **Access your app** at `https://yourusername.github.io/repositoryname/`

### Automatic Deployment
The application loads `bleuscore-js` directly from CDN, making it completely static and GitHub Pages compatible.

## 🎯 Usage Instructions

1. **Enter Predictions**: Add your machine translation outputs, one per line
2. **Enter References**: Add reference translations, one per line
3. **Configure Settings**: Choose n-gram order (1-5) and enable smoothing if needed
4. **Calculate**: Click "Calculate BLEU Score" or press `Ctrl+Enter`
5. **View Results**: See detailed BLEU metrics with visual representations

## 🔧 Technical Details

- **Frontend**: Vanilla JavaScript, HTML5, CSS3
- **Styling**: Tailwind CSS via CDN
- **BLEU Engine**: bleuscore-js npm package
- **Deployment**: Static files, GitHub Pages compatible
- **Browser Support**: All modern browsers

## 📝 Input Format

### Predictions
```
The cat is on the mat.
It is a good day.
Machine translation works well.
```

### References  
```
The cat is sitting on the mat.
Today is a beautiful day.
Machine translation performs well.
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [bleuscore-js](https://www.npmjs.com/package/bleuscore-js)
- Powered by the [BleuScore](https://github.com/shenxiangzhuang/bleuscore) Rust implementation
- UI design inspired by modern web applications

## 🔗 Related Links

- [BleuScore GitHub Repository](https://github.com/shenxiangzhuang/bleuscore)
- [BLEU Score Wikipedia](https://en.wikipedia.org/wiki/BLEU)
- [bleuscore-js npm package](https://www.npmjs.com/package/bleuscore-js) 